use anyhow::{Ok, Result};
use std::{
    collections::VecDeque,
    fmt::Result,
    sync::{atomic::AtomicUsize, Arc, Condvar, Mutex},
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
        todo!()
    }

    pub fn total_receivers(&self) -> usize {
        todo!()
    }

    pub fn total_queued_items(&self) -> usize {
        todo!()
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
        self.shared.senders.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// 克隆 sender
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

/// Drop sender
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        todo!()
    }
}

// 消费者离开时, 减少 receivers 的计数
impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
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
