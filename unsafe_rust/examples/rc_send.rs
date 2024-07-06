use std::{cell::RefCell, rc::Rc, thread};

#[derive(Debug, Default, Clone)]
struct Evil {
    data: Rc<RefCell<i32>>,
}

// Rc是!Send + !Sync的, 所以不能在线程中使用
// 这里为Evil强行实现Send, 将见证Rc的混乱!
unsafe impl Send for Evil {}

// 运行后,有一定的几率崩溃, 提示已经被可变借用了
fn main() {
    let v = Evil::default();
    let v1 = v.clone();
    let v2 = v.clone();

    let t1 = thread::spawn(move || {
        let v3 = v.clone();
        let mut data = v3.data.borrow_mut();
        *data += 1;
        println!("v3 = {:?}", data);
    });

    let t2 = thread::spawn(move || {
        let v4 = v1.clone();
        let mut data = v4.data.borrow_mut();
        *data += 1;
        println!("v4 = {:?}", data);
    });

    t2.join().unwrap();
    t1.join().unwrap();

    let mut data = v2.data.borrow_mut();
    *data += 1;
    println!("v2 = {:?}", data);
}
