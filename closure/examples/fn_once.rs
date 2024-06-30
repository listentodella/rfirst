// 由于call_once只用了self而非引用的形式
// 因此会发生消耗掉self, 导致该类型的闭包只能被调用一次
// 并且该类型的闭包,尽管可以捕获变量、获得所有权,但它不能改变被捕获的变量的值!
//pub trait FnOnce<Args> {
//    type Output;
//    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
//}

fn main() {
    let name = String::from("Tyr");
    // 这个闭包啥也不干，只是把捕获的参数返回去
    let c = move |greeting: String| (greeting, name);

    let result = c("hello".to_string());

    println!("result: {:?}", result);

    // 无法再次调用
    //let result = c("hi".to_string());

    // 但并不是说遇到move就一定会发生内部的所有权转移
    // 比如将被捕获的变量clone的话，还是可以被调用多次的
    main2();
}

fn main2() {
    let name = String::from("Tyr");

    // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce
    let c = move |greeting: String| (greeting, name.clone());

    // 尽管闭包内部使用了clone,保证被move的变量不被消耗(严格来说,被消耗的是被clone后的)
    // 但是被move的变量所有权仍然被转移给了闭包, 外部已经无法再使用了, 即便在闭包被调用前, name也无法使用
    //println!("name = {}", name);

    // 所以 c1 可以被调用多次
    println!("c1 call once: {:?}", c("qiao".into()));
    println!("c1 call twice: {:?}", c("bonjour".into()));

    // 注意这里的传参是&c, 而不是c本身, 有两种解释:
    // 1. 传参是&, 闭包自身所有权未转移, 所以可以被多次调用
    // 2. call_once的参数是 impl FnOnce, 但并不是只有 FnOnce 实现了它, 此时是以Fn的方法调用的
    println!("result: {:?}", call_once("hi".into(), &c));
    let result = c("hi".to_string());

    // 然而一旦它被当成 FnOnce 被调用，就无法被再次调用
    // 需要注意的是, c这里的传参是本体, 而不是引用,否则依旧可以多次调用c
    println!("result: {:?}", call_once("hi".into(), c));

    // 无法再次调用
    // let result = c("hi".to_string());

    // Fn 也可以被当成 FnOnce 调用，只要接口一致就可以
    println!("result: {:?}", call_once("hola".into(), not_closure));
}

fn call_once<F>(arg: String, c: F) -> (String, String)
where
    F: FnOnce(String) -> (String, String),
{
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}
