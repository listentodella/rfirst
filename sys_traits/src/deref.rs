use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    pub fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

fn main() {
    let mut buf = Buffer::new([1, 5, 2, 3, 4]);

    // 因为实现了 DerefMut 和 Deref,所以遇到.运算符
    //编译器会自动扩展为对应的deref()或 deref_mut()方法
    //(&mut buf).deref_mut().sort();
    buf.sort();

    println!("{:?}", buf);

    println!("Hello, world!");
}
