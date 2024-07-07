use std::{
    cell::RefCell,
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

struct Lock<T> {
    locked: AtomicBool,
    data: RefCell<T>,
}

impl<T> fmt::Debug for Lock<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

// SAFETY: 我们确信Lock<T>很安全,可以在多线程中共享
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: RefCell::new(data),
        }
    }

    pub fn lock(&self, op: impl FnOnce(&mut T)) {
        // 如果没拿到锁,就一直spin
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // 没有这一句也不影响正确性
            // 性能优化: compare_exchange 需要独占访问
            // 当拿不到锁时,我们先不停检测 locked 的状态
            // 知道其unlocked后,再尝试拿锁
            while self.locked.load(Ordering::Relaxed) == true {}
        }

        // 运行到这里说明已经拿到锁并加锁了
        op(&mut self.data.borrow_mut());

        // 解锁
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let data = Arc::new(Lock::new(0));

    let data1 = data.clone();
    let t1 = thread::spawn(move || {
        data1.lock(|d| *d += 10);
    });

    let data2 = data.clone();
    let t2 = thread::spawn(move || {
        data2.lock(|d| *d *= 10);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("data: {:?}", data);
}
