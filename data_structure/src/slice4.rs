use std::iter::FromIterator;

fn main() {
    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];
    let s = String::from("hello");
    let s1 = &arr[1..3];
    let s2 = &vec[1..3];
    //&str本身就是一个特殊的slice
    let s3 = &s[1..3];
    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);

    // &[char] 和 &[char] 就是切片之间的比较规则
    assert_eq!(s1, s2);
    // &[char] 和 &str 不能直接对比, 我们把s3变成Vec<char>
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());
    // &[char] 可以通过迭代器转换成 String, String 和 &str 可以直接比较
    assert_eq!(String::from_iter(s2), s3);
}
