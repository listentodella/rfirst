use std::{collections::HashMap, mem::size_of_val};

// 闭包无论是否有形参、局部变量，都不会影响其大小
// 如果闭包没有捕获任何变量/引用，则长度为0
// 如果闭包使用move关键字捕获变量，则长度为所有被捕获变量的大小之和

fn main() {
    // 长度为0
    let c1 = || println!("Hello, world!");

    // 和形参无关, 长度也为0
    let c2 = |i: i32| println!("The number is {}", i);

    let name = String::from("leo");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    // 捕获了一个引用, 长度为8
    let c3 = || println!("hello: {}", name);

    // 用move捕获移动的数据, 闭包长度为72, name1(24) + table(48)
    let c4 = move || println!("hello: {}, {:?}", name1, table);

    let name2 = name.clone();
    // 和局部变量无关, 捕获一个String, 闭包长度为24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("LEO");
        println!("hello: {}, {}, {}", name2, name3, x);
    };

    println!(
        "size_of HashMap: {}",
        std::mem::size_of::<HashMap<&str, &str>>()
    );
    println!("size_of String: {}", std::mem::size_of::<String>());

    println!("c1 size: {}", size_of_val(&c1));
    println!("c2 size: {}", size_of_val(&c2));
    println!("c3 size: {}", size_of_val(&c3));
    println!("c4 size: {}", size_of_val(&c4));
    println!("c5 size: {}", size_of_val(&c5));
    println!("main size: {}", size_of_val(&main));
}
