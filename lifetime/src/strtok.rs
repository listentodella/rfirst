// 因为最终的返回值与 &str 有关,而不是 &mut &str

//error!因为这个相当于告诉编译器, 返回值与 &mut &str 相关联了,
//这意味着只要返回值还在被使用,那么作为入参s的 &mut &str就不能被被释放
//尽管这不妨碍可以得到正确的返回值,但是入参s,即 &mut &str的生命周期被延长了
// 导致 &mut s1 活到了 strtok 函数之外,即hello, 而它与最后的println里的不可变s1同时出现,
// 违背了 不可变与可变引用不可同时出现的规则
// 不过如果把s1放到另一句println里,就不会有这个冲突了
//pub fn strtok<'a>(s: &'a mut &str, delimiter: char) -> &'a str {
//ok
//pub fn strtok<'b, 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str {
// 利用编译器自己扩展成上述形式
pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
        // 直接使用 len 返回的是字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
    //println!("hello is: {}, s: {}", hello, s);
    //println!("s1: {}", s1);
}
