use lazy_static::lazy_static;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

//MutexGuard 除了提供了Deref,还通过Drop trait来保证使用到的内存以外的资源在退出时进行释放
// 由于MutexGuard 只允许Sync不允许Send,因此只能把MutexGuard的引用传给另一个线程
// 而不能将整个MutexGuard传递给另一个线程
//impl<T: ?Sized> !Send for MutexGuard<'_, T> {}
//unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

// lazy_static 宏可以生成复杂的static对象
lazy_static! {
    // 一般情况下, Mutex和Arc一起在多线程环境下提供对共享内存的使用
    // 如果你把Mutex声明称static,其生命周期是静态的, 不需要Arc
    static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> =
    Mutex::new(HashMap::new());
}

fn main() {
    // 用Arc来提供并发环境下的共享所有权(使用引用计数)
    let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..32 {
        let m = metrics.clone();
        thread::spawn(move || {
            // 尝试上锁
            let mut g = m.lock().unwrap();
            // 此时只有拿到MutexGuard的线程才能访问HashMap
            let data = &mut *g;
            // Cow实现了很多数据结构的 From trait
            // 所以我们可以用"hello".into()来生成一个Cow
            let entry = data.entry("hello".into()).or_insert(0);
            *entry += 1;
            // MutexGuard 被Drop, 锁被释放
        });
    }

    thread::sleep(Duration::from_millis(100));

    println!("metrics:{:?}", metrics.lock().unwrap());
}
