fn main() {
    let s1 = "Hello, world!";
    println!("first word of s1 :{}", first(s1));
}

//clippy 提示不需要,编译器可以识别
//fn first<'a>(s: &'a str) -> &'a str {
fn first(s: &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}
