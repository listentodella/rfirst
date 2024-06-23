use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

/*
查看spawn的参数类型要求:
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
*/

// Rc 既不是 Send 也不是 Sync，所以不能在线程间共享
//impl<T: ?Sized, A: Allocator> !Send for Rc<T, A> {}
//impl<T: ?Sized, A: Allocator> !Sync for Rc<T, A> {}
// fn rc_is_not_send_and_sync() {
// let a = Rc::new(1);
// let b = a.clone();
// let c = a.clone();
// thread::spawn(move || {
// println!("c = {:?}", c);
// });
// }

// RefCell<T> 实现了 Send，但不是 Sync, 意味着它能移动到线程间, 但不能共享
//unsafe impl<T: ?Sized> Send for RefCell<T> where T: Send {}
//impl<T: ?Sized> !Sync for RefCell<T> {}
fn refcell_is_send() {
    let a = RefCell::new(1);
    thread::spawn(move || {
        println!("a = {:?}", a);
    });
}

// Arc<T> 实现了 Send 和 Sync, 所以可以安全的在线程间共享
//unsafe impl<T: ?Sized> Send for Arc<T> where T: Send + Sync {}
//unsafe impl<T: ?Sized> Sync for Arc<T> where T: Send + Sync {}
// 但即便如此包装了一下 RefCell<T> 也还是不能在线程间共享
// 这是因为Arc内部的数据是共享的,需要支持Sync的数据结构
// fn ref_cell_is_not_sync() {
//     let a = Arc::new(RefCell::new(1));
//     let b = a.clone();
//     thread::spawn(move || {
//         println!("b = {:?}", b);
//     });
// }

// Mutex<T> 实现了 Send 和 Sync, 所以可以安全的在线程间共享
//unsafe impl<T> Send for Mutex<T> where T: Send {}
//unsafe impl<T> Sync for Mutex<T> where T: Send {}
fn arc_mutex_is_send_and_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = a.lock().unwrap();
        *g += 1;
    }

    handle.join().unwrap();
    println!("a = {:?}", a);
}

fn main() {
    arc_mutex_is_send_and_sync();
    println!("Hello, world!");
}
