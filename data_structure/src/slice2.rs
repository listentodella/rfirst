use std::fmt;

fn print_slice<T: fmt::Debug>(s: &[T]) {
    println!("{:?}", s);
}

fn print_slice1<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

fn main() {
    let v = vec![1, 2, 3, 4];

    // Vec 实现了Deref, &Vec<T>会被自动解引用成 &[T],符合接口定义
    print_slice(&v);
    // 直接是 &[T], 符合接口定义
    print_slice(&v[..]);
    // &Vec<T> 支持 AsRef<T>
    print_slice1(&v);
    // &[T] 支持 AsRef<T>
    print_slice1(&v[..]);
    // Vec<T> 支持 AsRef<T>
    print_slice1(v);

    let arr = [1, 2, 3, 4];
    // 数据虽没有实现Deref,但是它的解引用就是&[T]
    print_slice(&arr);
    print_slice(&arr[..]);
    print_slice1(&arr);
    print_slice1(&arr[..]);
    print_slice1(arr);
}
