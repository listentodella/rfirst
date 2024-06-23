use std::mem::{align_of, size_of};

#[allow(dead_code)]
struct S1 {
    a: u8,
    b: u16,
    c: u8,
}

#[allow(dead_code)]
struct S2 {
    a: u8,
    c: u8,
    b: u16,
}

fn main() {
    // 编译器会自动优化至最佳大小,但也意味着,成员不是按照定义的顺序在内存中分布
    println!("sizeof S1: {}, S2: {}", size_of::<S1>(), size_of::<S2>());
    println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
}
