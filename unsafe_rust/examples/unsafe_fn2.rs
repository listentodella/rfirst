fn main() {
    let data = b"Hello, world!";
    let s1 = unsafe { std::str::from_utf8_unchecked(data) };

    // 其实safe的from_utf8()内部调用了unsafe的from_utf8_unchecked()
    // 它多做了一次检查，所以效率会低一些, 但它是安全的
    let s2 = std::str::from_utf8(data).unwrap();

    println!("{}", s1);
    println!("{}", s2);
}
