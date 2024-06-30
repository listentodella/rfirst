//怎么理解 FnOnce 的 Args 泛型参数呢？Args 又是怎么和 FnOnce 的约束，比如 FnOnce(String) 这样的参数匹配呢
// 下面的例子模拟了FnOnce的使用, 尽管不是那么完全

struct ClosureOnce<Caputred, Args, Output> {
    // 捕获的数据
    captured: Caputred,
    // closure 的执行代码
    func: fn(Args, Caputred) -> Output,
}

impl<Caputred, Args, Output> ClosureOnce<Caputred, Args, Output> {
    // 模拟 FnOnce 的 call_once, 直接消耗 self
    fn call_once(self, args: Args) -> Output {
        (self.func)(args, self.captured)
    }
}

fn greeting_code1(args: (String,), captured: (String,)) -> (String, String) {
    (args.0, captured.0)
}

fn greeting_code2(args: (String, String), captured: (String, u8)) -> (String, String, String, u8) {
    (args.0, args.1, captured.0, captured.1)
}

fn main() {
    let name = "leo233".to_string();

    //模拟变量捕获
    let c = ClosureOnce {
        captured: (name,),
        func: greeting_code1,
    };

    // 模拟闭包调用,这里和FnOnce不完全一样, 传入的是一个 tuple 来匹配 Args 参数
    println!("{:?}", c.call_once(("hola".into(),)));
    // 调用一次后无法再次调用
    //println!("{:?}", c.call_once(("hola".into(),)));

    // 更复杂一些的复杂的闭包
    let c1 = ClosureOnce {
        captured: ("LEO".into(), 18),
        func: greeting_code2,
    };
    println!("{:?}", c1.call_once(("hola".into(), "hola".into())));
}
