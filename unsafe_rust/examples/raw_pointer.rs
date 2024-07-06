fn main() {
    let mut age = 18;

    //不可变指针
    let r1 = &age as *const i32;
    //可变指针
    let r2 = &mut age as *mut i32;

    // 使用裸指针,可以绕过 immutable / mutable borrow rules

    unsafe {
        println!("r1:{}, r2:{}", *r1, *r2);
    }
}

//fn immutable_mutable_cant_coexist() {
//    let mut age = 18;
//    let r1 = &age;
//    let r2 = &mut age; // error: mutable borrow occurs here
//
//    // 编译器会报错，因为不可变指针和可变指针不能同时存在
//    println!("r1:{}, r2:{}", *r1, *r2);
//}
