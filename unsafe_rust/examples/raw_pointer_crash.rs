fn main() {
    let r1 = 0xdeadbeef as *mut i32;

    println!("so far so good!");

    unsafe {
        // 程序崩溃
        *r1 += 1;
        println!("{:?}", *r1);
    }
}
