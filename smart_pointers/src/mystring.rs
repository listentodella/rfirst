use std::{fmt, ops::Deref, str};

// 1个字节表示字符串的长度
// 30个字节表示字符串内容
// 1个字节的tag

const MINI_STRING_MAX_LEN: usize = 30;

// MyString 里, String有3个word,共24字节,所以它以8字节对齐
// 所以enum的tag+padding 最少8字节,以使整个结构占32字节
// MiniString可以最多有30字节(再加上1字节len+1字节tag),共32字节
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    // 这里 new 接口不暴露出去，保证传入的 v 的字节长度小于等于 30
    fn new(s: impl AsRef<str>) -> Self {
        let bytes = s.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        //由于生成MiniString的接口是隐藏的,它只能来自字符串,所以下面这行是安全的
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //由于已经实现了 Deref, 可以直接得到一个 &str 输出
        write!(f, "MiniString({})", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MyString::Inline(ref s) => s.deref(),
            MyString::Standard(ref s) => s.deref(),
        }
    }
}

// impl From<&str> for MyString {
//     fn from(s: &str) -> Self {
//         match s.len() > MINI_STRING_MAX_LEN {
//             true => MyString::Standard(s.to_owned()),
//             false => MyString::Inline(MiniString::new(s)),
//         }
//     }
// }

// impl From<String> for MyString {
//     fn from(s: String) -> Self {
//         match s.len() > MINI_STRING_MAX_LEN {
//             true => MyString::Standard(s),
//             false => MyString::Inline(MiniString::new(s)),
//         }
//     }
// }
// 使用泛型参数 T: AsRef<str> 实现 From<T> for MyString
impl<T> From<T> for MyString
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        match s.as_ref().len() > MINI_STRING_MAX_LEN {
            true => MyString::Standard(s.as_ref().to_owned()),
            false => MyString::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!(
        "MyString size: {} bytes, MiniString size: {} bytes",
        len1, len2
    );

    let s1: MyString = "hello world".into();
    let s2: MyString = "x".repeat(34).into();
    //let s2: MyString = "HELLLO".into();

    //debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);

    //display 输出
    println!(
        "s1: {}, s1 len: {}, s1 char count: {}, s2: {}, s2 len: {}, s2 char count: {}",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );

    //MyString可以使用一切 &str 接口, 感谢 Rust 的自动Deref!
    assert!(s1.ends_with("world"));
    assert!(s2.ends_with("x"));

    let s = String::from("这是一个超过了三十个字节的很长很长的字符串");
    println!("s: {:p}", &*s);
    // From<T: AsRef<str>> 的实现会导致额外的复制
    let s3: MyString = s.into();
    println!("s3: {:p}", &*s3);

    let mut s4: MyString = "Hello!".into();
    println!("s4 before push_str: {:?}", s4);
    s4.push_str("这是一个超过了三十个字节的很长很长的字符串");
    println!("s4 after push_str: {:?}", s4);
}

impl MyString {
    fn push_str(&mut self, s: &str) {
        match self {
            MyString::Inline(v) => {
                let self_len = v.len as usize;
                let len = s.len();
                if len + self_len > MINI_STRING_MAX_LEN {
                    let mut owned = v.deref().to_string();
                    owned.push_str(s);
                    *self = MyString::Standard(owned);
                } else {
                    let total = self_len + len;
                    v.data[self_len..total].copy_from_slice(s.as_bytes());
                    v.len = total as u8;
                }
            }
            MyString::Standard(v) => v.push_str(s),
        }
    }
}
