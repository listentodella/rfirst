use std::ops::Mul;

fn main() {
    let c1 = curry(5);
    println!("5 multiply 2 is {}", c1(2));

    let adder2 = curry(std::f64::consts::PI);
    println!("pi multiply 4^2 is {}", adder2(4. * 4.));
}

// 该泛型函数接收一个参数x,要求这个类型T必须实现Mul<Output=T> + Copy
// 然后该函数的返回值是一个闭包，该闭包接收一个同类型的参数,并返回相同的类型
fn curry<T>(x: T) -> impl Fn(T) -> T
where
    T: Mul<Output = T> + Copy,
{
    move |y| x * y
}
