use std::fmt;

fn print_slice(s: &str) {
    println!("{:?}", s);
}

fn print_slice1<T>(s: T)
where
    T: AsRef<str>,
{
    println!("{:?}", s.as_ref());
}

fn print_slice2<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

fn main() {
    let v = String::from("hello");

    // &String 会被解引用成 &str
    print_slice(&v);
    // &v[..] 和 v.as_str() 效果相同,都会解引用成 &str
    print_slice(&v[..]);

    //String 实现了 AsRef<str>
    print_slice1(&v);
    print_slice1(&v[..]);
    print_slice1(v.clone());

    // String 也实现了 AsRef<[u8]>,所以下面的代码成立
    // 不过不会以字符串形式打印
    print_slice2(&v);
    print_slice2(&v[..]);
    print_slice2(v.clone());
}
