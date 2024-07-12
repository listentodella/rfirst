use futures::executor::block_on;
use std::future::Future;

async fn say_hello1(name: &str) -> usize {
    println!("Hello, {}!", name);
    42
}

// async fn 关键字相当于一个返回 impl Future<Output> 的语法糖
// 如果不用async修饰的话,就得写成如下形式
fn say_hello2<'fut>(name: &'fut str) -> impl Future<Output = usize> + 'fut {
    async move {
        println!("Hello, {}!", name);
        42
    }
}

#[tokio::main]
async fn main() {
    let name1 = "Leo".to_string();
    let name2 = "Rustacean".to_string();

    say_hello1(&name1).await;
    say_hello2(&name2).await;

    //Future除了可以用await来执行外,还可以直接用executor来执行
    block_on(say_hello1(&name1));
    block_on(say_hello2(&name2));
}
