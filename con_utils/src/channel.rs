use anyhow::Result;
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
        todo!()
    }

    pub fn total_senders(&self) -> usize {
        todo!()
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

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        todo!()
    }
}

/// 创建一个 unbounded channel
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};
    // 先省略test case
}
