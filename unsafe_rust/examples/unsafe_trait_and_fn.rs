/// 实现这个trait的开发者要保证,实现是内存安全的.
unsafe trait Foo {
    fn safe_foo(&self);
    unsafe fn unsafe_foo(&self);
}

trait Bar {
    // 调用这个函数的人要保证，调用是安全的
    unsafe fn bar(&self);
}

struct Nonsense;

/// 实现unsafe的trait,前面也要加上unsafe关键字
unsafe impl Foo for Nonsense {
    fn safe_foo(&self) {
        println!("safe foo!");
    }
    unsafe fn unsafe_foo(&self) {
        println!("unsafe foo!");
    }
}

/// trait 自身属于safe的,就不用unsafe关键字
impl Bar for Nonsense {
    /// 但实现unsafe的trait的方法,前面也要加上unsafe关键字
    unsafe fn bar(&self) {
        println!("bar!");
    }
}

unsafe fn unsafe_function() {
    println!("unsafe function!");
}

fn main() {
    let nonsense = Nonsense;

    // 调用者无需关心unsafe trait的safe方法
    nonsense.safe_foo();

    // 调用者需要关心unsafe trait的unsafe方法
    unsafe {
        nonsense.unsafe_foo();
    }

    // 调用者需要关心safe trait的unsafe方法
    unsafe { nonsense.bar() };

    // 调用者需要关心unsafe函数
    unsafe { unsafe_function() };
}
