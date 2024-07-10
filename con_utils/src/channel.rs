use anyhow::{anyhow, Result};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
};

/// VecDeque 用于存储消息, 并且可自动扩容
/// Mutex 则用于互斥保护
/// Condvar 用于线程间同步,通知
/// AtomicUsize 用于计数 senders 和 receivers
struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

/// Sender
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

/// Receiver
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    cache: VecDeque<T>,
}

impl<T> Sender<T> {
    /// 生产者写入一个数据
    pub fn send(&mut self, t: T) -> Result<()> {
        // 如果没有消费者了,写入时出错
        if self.total_receivers() == 0 {
            return Err(anyhow!("no more receivers!"));
        }

        // 加锁,访问 VecDeque, 压入数据,然后立刻释放锁
        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let is_empty = inner.is_empty();
            inner.push_back(t);
            is_empty
        };

        // 通知任意一个被挂起等待的消费者有数据
        // 前提是此前队列为空
        if was_empty {
            self.shared.available.notify_one();
        }

        Ok(())
    }

    pub fn total_receivers(&self) -> usize {
        // 这里使用 SeqCst, 保证所有线程看到同样顺序的对recivers的操作
        self.shared.receivers.load(Ordering::SeqCst)
    }

    pub fn total_queued_items(&self) -> usize {
        self.shared.queue.lock().unwrap().len()
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        // 先尝试从cache中读取,如果有数据,直接返回
        // 如果没有数据,要么cache为一开始的空,要么已经被人读光了
        // 此时就需要从原始的queue中读取数据了
        // pop_front()一次只能读取一笔数据,因此只要适当累积
        // 消费者就能高效地从cache中无锁读取
        // 生产者还是依旧会有锁写入,但竞争频率会减少
        if let Some(t) = self.cache.pop_front() {
            return Ok(t);
        }

        // 拿到队列的锁
        let mut inner = self.shared.queue.lock().unwrap();
        // 这里的loop主要是针对还有Sender但没有数据的情况
        // 此时进入wait后如果被唤醒,应该重新去匹配模式
        loop {
            match inner.pop_front() {
                // 读到数据返回,锁被释放
                Some(t) => {
                    // 如果当前队列中还有数据,那么就把消费者自身缓存的队列(空)和共享队列swap一下
                    // 这样之后再读取,就可以从 self.queue 中无锁读取
                    if !inner.is_empty() {
                        // swap的效率很高,交换的是指针,而没有对数据进行交换
                        std::mem::swap(&mut self.cache, &mut inner);
                    }
                    return Ok(t);
                }
                // 读不到数据并且也没有生产者了,释放锁并返回错误
                None if self.total_senders() == 0 => return Err(anyhow!("no more senders!")),
                // 读不到数据,把锁交给Condvar,它会释放锁并挂起线程等待
                // 等到通知后,它会再获取锁,得到一个MutexGuard
                None => {
                    // 当Condvar被唤醒后会返回MutexGuard
                    // 我们可以loop回去拿数据
                    // 这就是为什么Condvar要在loop里使用
                    inner = self
                        .shared
                        .available
                        .wait(inner)
                        .map_err(|_| anyhow!("lock poisoned!"))?;
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}

// 要考虑到用户可能会尝试用iterator的方式调用Receiver
impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // ok()可以将Result<T>转换为Option<T>
        self.recv().ok()
    }
}

/// 因为是mpsc,支持多个生产者,所以要允许它clone
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: self.shared.clone(),
        }
    }
}

/// Drop sender
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let exist = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        // 然而如果sender都不在了,而接收方全部是处于wait状态
        // 那么接收者无人唤醒,就会一直阻塞了!
        if exist <= 1 {
            // 其实只有一个recv, notify_one 和 all 的效果是一样的
            //self.shared.available.notify_one();
            self.shared.available.notify_all();
        }
    }
}

// 消费者离开时, 减少 receivers 的计数
impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

/// 创建一个 unbounded channel
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            cache: VecDeque::with_capacity(INITAL_SIZE),
        },
    )
}

const INITAL_SIZE: usize = 32;
impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    /// case1:Sender可以产生数据,Recv可以消费数据
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world!".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello world!");
    }

    #[test]
    /// case2:支持MPSC
    fn mp_should_work() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();
        let t = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t1 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(3).unwrap();
        });

        for handle in [t, t1, t2] {
            handle.join().unwrap();
        }

        let mut ret = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
        //由于数据到达的顺序不确定,所以排序后再比较
        ret.sort_unstable();
        assert_eq!(ret, [1, 2, 3]);
    }

    #[test]
    #[allow(clippy::all)]
    /// case3:Recv可能被阻塞(为空时)
    /// 可以通过检测"线程是否退出"来间接判断线程是否被阻塞
    fn receiver_should_be_blocked_when_nothing_to_read() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();
        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                // 如果读到数据,确保它和发送的数据一致
                assert_eq!(idx, i)
            }
            // 读不到数据应该休眠,所以执行不到这一句,如果执行到说明逻辑出错
            assert!(false)
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
            //防止所有sender都离开
            loop {}
        });

        // 1ms足以让消费者发送100个消息, 消费者消费完100个消息并阻塞
        thread::sleep(Duration::from_millis(1));

        // 再次发送数据,唤醒消费者
        for i in 100..200usize {
            s1.send(i).unwrap();
        }

        // 留点时间让 receiver 处理
        thread::sleep(Duration::from_millis(1));

        // 如果receiver被正常唤醒处理,那么队列里的数据会都被读完
        assert_eq!(s1.total_queued_items(), 0);
    }

    #[test]
    /// case4:Recv需要知道另一端没有Sender的情况
    fn last_sender_drop_should_error_when_receive() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();

        // sender即用即抛
        for mut sender in senders {
            thread::spawn(move || {
                sender.send("hello".to_string()).unwrap();
                // sender 再次被丢弃
            })
            .join()
            .unwrap();
        }

        // 理论上即便没有sender,接收者依然可以接受已经在队列里的数据
        for _ in 0..total {
            r.recv().unwrap();
        }

        // 然而,尝试读取更多数据需要出错
        assert!(r.recv().is_err())
    }

    #[test]
    /// case5:Sender需要知道另一端没有Recv的情况
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s1 = s.clone();
            (s1, s)
        };
        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }

    #[test]
    fn receiver_should_be_notified_when_all_senders_exit() {
        let (s, mut r) = unbounded::<usize>();
        // 用于两个线程同步
        let (mut sender, mut receiver) = unbounded();
        let t1 = thread::spawn(move || {
            // 保证 r.recv()先于t2的drop执行
            // 如果t2先执行,那么也只能在t2的receiver.recv()处阻塞等待
            // t2想要继续执行就必须等待此处的sender.send()
            // 如果t1先执行,那么sender本就在t2的recv之前执行

            // TODO:尽管是如此设定,但recv唤醒处理,就一定赶不上吗?
            sender.send(0).unwrap();
            assert!(r.recv().is_err());
        });

        thread::spawn(move || {
            receiver.recv().unwrap();
            drop(s);
        });

        t1.join().unwrap();
    }
}
