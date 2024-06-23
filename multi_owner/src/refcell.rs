use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);
    //通过下面的花括号,明确缩小v的作用域以减少其生命周期
    //否则会出现运行时的panic,因为可变与不可变引用同时作用了
    {
        let mut v = data.borrow_mut();
        *v += 1;
    }

    println!("data:{:?}", data.borrow());
}
