use std::thread;

static mut COUNTER: usize = 1;

fn main() {
    let t1 = thread::spawn(move || {
        unsafe { COUNTER += 10 };
    });

    let t2 = thread::spawn(move || {
        unsafe { COUNTER *= 10 };
    });

    t2.join().unwrap();
    t1.join().unwrap();

    // 两个线程同时访问同一个全局变量,而且没有任何保护, 其行为是不安全的
    unsafe { println!("COUNTER: {}", COUNTER) };
}
