use std::fmt;

fn print_slice(s: &str) {
    println!("{:?}", s);
}

// str 实现了 AsRef<[u8]>, AsRef<str>, AsRef<OsStr>, AsRef<Path>
// 编译器遇到 AsRef<U> 时,有4种选择反而无法推断
// 直接调用会出现编译报错, 但是调用时如果进行一些限定,可以帮助编译器进行推断
fn print_slice1<T, U>(s: T)
where
    T: AsRef<U>,
    U: fmt::Debug + ?Sized,
{
    println!("{:?}", s.as_ref());
}

// str 实现了 AsRef<[u8]>, AsRef<str>, AsRef<OsStr>, AsRef<Path>
// 所以编译器遇到 AsRef<[U]> 时,可以推断出 AsRef<[u8]>
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
    println!("=======below print slice1 =========");
    //String 实现了 AsRef<str>
    print_slice1::<_, [u8]>(&v);
    print_slice1::<_, str>(&v[..]);
    print_slice1::<_, str>(v.clone());

    println!("=======below print slice2 =========");
    // String 也实现了 AsRef<[u8]>,所以下面的代码成立
    // 不过不会以字符串形式打印
    print_slice2(&v);
    print_slice2(&v[..]);
    print_slice2(v.clone());
}
