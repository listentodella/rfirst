// 从定义来看, FnMut 是一个 trait, 它继承了 FnOnce, 并添加了 call_mut 方法
// 并且 call_mut 的参数类型是 &mut self, 所以可以修改 self 的值
// 并且是可以多次调用的, 因为是self的引用, 而非直接消耗self
// 但是由于继承了 FnOnce, 所以 call_once 方法也被继承, 当使用 call_once 时, 则只能调用一次了

//pub trait FnMut<Args>: FnOnce<Args> {
//    extern "rust-call" fn call_mut(
//        &mut self,
//        args: Args
//    ) -> Self::Output;
//}

fn main() {
    let mut name = "leo233".to_string();
    let mut name1 = "LEO".to_string();

    // 捕获 &mut name
    let mut c = || {
        name.push_str(" from c");
        println!("name: {}", name);
    };

    // 捕获 mut name1
    let mut c1 = move || {
        name1.push_str(" from c1");
        println!("name1: {}", name1);
    };

    c();
    c1();

    call_mut(&mut c);
    call_mut(&mut c1);
    // 还可以继续调用
    c();
    c1();

    // 这里传的是&mut, 闭包的所有权没有发生转移, 所以可以多次调用
    call_once(&mut c);
    call_once(&mut c1);

    c();
    c1();

    call_once(c);
    call_once(c1);
    // 如果直接传的是本体, 闭包自身就被消费掉了, 所以不能再调用
    //c();
    //c1();
}

// 作为参数时, FnMut 也要显式使用 mut 或者 &mut
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// 不过这样写会消耗掉这个闭包, 如果想多次调用, 传参时需要使用 &mut
//fn call_mut(mut c:impl FnMut()) {
//    c();
//}

// 也可以这样表示
//fn call_mut2<F>(mut c: F)
//where
//    F: FnMut(),
//{
//    c();
//}

fn call_once(c: impl FnOnce()) {
    c();
}
