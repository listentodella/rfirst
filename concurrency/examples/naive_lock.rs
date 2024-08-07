use std::{
    cell::RefCell,
    fmt::{self, Debug},
    sync::Arc,
    thread,
};

struct Lock<T> {
    locked: RefCell<bool>,
    data: RefCell<T>,
}

impl<T> fmt::Debug for Lock<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

// SAFETY:我们确信 Lock<T> 很安全,可以在多线程中共享
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: RefCell::new(false),
            data: RefCell::new(data),
        }
    }

    pub fn lock(&self, op: impl FnOnce(&mut T)) {
        // 如果一直没拿到锁,就一直spin
        while *self.locked.borrow() != false {}

        // 拿到锁, 立即上锁
        *self.locked.borrow_mut() = true;

        // do something
        op(&mut self.data.borrow_mut());

        // 解锁
        *self.locked.borrow_mut() = false;
    }
}

fn main() {
    let data = Arc::new(Lock::new(0));
    let data1 = data.clone();
    let t1 = thread::spawn(move || {
        data1.lock(|v| *v += 10);
    });

    let data2 = data.clone();
    let t2 = thread::spawn(move || {
        data2.lock(|v| *v += 20);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("data:{:?}", data);
}
