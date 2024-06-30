// 从定义来看, Fn 是一个 trait, 它继承了 FnMut, 并添加了 call 方法
// 并且 call 的参数类型是 &self, 不可修改 self 的值
// 并且是可以多次调用的, 因为是self的引用, 而非直接消耗self
// 但是由于继承了 FnMut, 所以 call_mut/call_once 方法也被继承, 当使用 call_once 时, 则只能调用一次了

//pub trait Fn<Args>: FnMut<Args> {
//    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
//}

fn main() {
    let v = vec![0u8; 1024];
    let v1 = vec![0u8; 1023];

    // 不移动所有权
    let mut c = |x: u64| v.len() as u64 + x;
    // 移动所有权
    let mut c1 = move |x: u64| v1.len() as u64 + x;

    println!("direct call: {}", c(2));
    println!("direct call: {}", c1(2));

    println!("call: {}", call(3, &c));
    println!("call: {}", call(3, &c1));

    println!("call_mut: {}", call_mut(4, &mut c));
    println!("call_mut: {}", call_mut(4, &mut c1));

    println!("call_once: {}", call_once(5, c));
    println!("call_once: {}", call_once(5, c1));
}

fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
    c(arg)
}

fn call_mut(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
    c(arg)
}

fn call_once(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}
