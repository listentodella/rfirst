use std::{slice::from_raw_parts_mut, str::from_utf8_unchecked_mut};

fn my_split(s: &str, sep: char) -> Option<(&str, &str)> {
    let pos = s.find(sep);
    pos.map(|pos| {
        let len = s.len();
        let sep_len = sep.len_utf8();

        // SAFETY:pos是find得到的,它位于字符的边界处, 同样 pos + sep_len 也是如此
        // 因此下面的代码是安全的
        unsafe { (s.get_unchecked(0..pos), s.get_unchecked(pos + sep_len..len)) }
    })
}

fn main() {
    let mut s = "我爱你！中国".to_string();
    let r = s.as_mut();
    if let Some((s1, s2)) = my_split(r, '！') {
        //if let Some((s1, s2)) = my_split(r, '.') {
        println!("{} {}", s1, s2);
    }
}

#[allow(dead_code)]
fn my_split_mut(s: &mut str, sep: char) -> Option<(&mut str, &mut str)> {
    let pos = s.find(sep);

    pos.map(|pos| {
        let ptr = s.as_mut_ptr();
        let len = s.len();
        let sep_len = sep.len_utf8();

        let s1 = unsafe { from_raw_parts_mut(ptr, len) };
        let s2 = unsafe { from_raw_parts_mut(ptr.add(pos + sep_len), len - pos - sep_len) };

        unsafe { (from_utf8_unchecked_mut(s1), from_utf8_unchecked_mut(s2)) }
    })
}
