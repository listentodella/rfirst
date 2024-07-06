use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex, thread};

// 使用 lazy_static 初始化复杂的结构
lazy_static! {
    // 使用 Mutex/RwLock 来提供安全的并发写访问
    static ref STORE: Mutex<HashMap<&'static str, &'static [u8]>> = Mutex::new(HashMap::new());
}

// 对于复杂的结构,无法直接使用Atomic的话,可以使用 Mutex/RwLock 来提供安全的并发写访问
fn main() {
    let t1 = thread::spawn(move || {
        let mut store = STORE.lock().unwrap();
        store.insert("key1", b"value1");
    });

    let t2 = thread::spawn(move || {
        let mut store = STORE.lock().unwrap();
        store.insert("key2", b"value2");
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Store: {:?}", STORE.lock().unwrap());
}
