use anyhow::{Ok, Result};
use std::{
    collections::VecDeque,
    fmt::Result,
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
}

impl<T> Shared<T> {
    /// 生产者写入一个数据
    pub fn send(&mut self, t: T) -> Result<()> {
        // 如果没有消费者了,写入时出错
        if self.total_receivers() == 0 {
            return Err(anyhow!("no more receivers!"));
        }

        // 加锁,访问 VecDeque, 压入数据,然后立刻释放锁
        let was_empty = {
            let mut inner = self.queue.lock().unwrap();
            let is_empty = inner.is_empty();
            inner.push_back(t);
            is_empty
        };

        // 通知任意一个被挂起等待的消费者有数据
        // 前提是此前队列为空
        if was_empty {
            self.available.notify_one();
        }

        Ok(())
    }

    pub fn total_receivers(&self) -> usize {
        // 这里使用 SeqCst, 保证所有线程看到同样顺序的对recivers的操作
        self.shared.recvers.load(Ordering::SeqCst)
    }

    pub fn total_queued_items(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        // 拿到队列的锁
        let mut inner = self.shared.queue.lock().unwrap();
        // 这里的loop主要是针对还有Sender但没有数据的情况
        // 此时进入wait后如果被唤醒,应该重新去匹配模式
        loop {
            match inner.pop_front() {
                // 读到数据返回,锁被释放
                Some(t) => return Ok(t),
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

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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
        self.shared.senders.fetch_sub(1, Ordering::AcqRel);
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
        Receiver { shared },
    )
}

const INITAL_SIZE: usize = 32;
impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(0),
            receivers: AtomicUsize::new(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};
    // 先省略test case
}
