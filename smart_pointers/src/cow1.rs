use std::borrow::Cow;

use url::Url;

fn main() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();

    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    // 因为k,v都是Cow<str>, 它们用起来感觉和&str或String一样
    // 此刻,它们都是 Borrowed
    println!("k:{}, v:{}", k, v);

    // 当修改发生时,k变成Owned
    k.to_mut().push_str("_lala");

    print_pairs((k, v));

    print_pairs(pairs.next().unwrap());
    // 在处理extra=hello%20world时, value被处理成 "hello world"
    // 所以, 这里v是Owned
    print_pairs(pairs.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("k:{}, v:{}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(s) => format!("Borrowed({})", s),
        Cow::Owned(s) => format!("Owned({})", s),
    }
}
