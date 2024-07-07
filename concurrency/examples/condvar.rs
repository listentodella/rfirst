use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let t1 = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        eprintln!("I'm a happy worker!");
        //notify main thread
        cvar.notify_one();

        // 要么主动调用drop, 要么不要用死循环
        // 否则 started 一直持有锁
        // 即便 Condvar 已经进行了 notify_one
        // 在wait的线程也拿不到 started这个 MutexGuard
        // 导致一直阻塞
        drop(started);

        loop {
            thread::sleep(Duration::from_secs(1));
            println!("working...");
        }
    });

    // wait for thread's notification
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    eprintln!("worker started!");

    // 如果没有这里的join
    // 主线程会在得到后面的notify后,并且退出了循环
    // 之后main会完成执行, 直接会销毁 t1, 即便loop没有结束
    //t1.join().unwrap();
}
