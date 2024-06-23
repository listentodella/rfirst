use regex::Regex;
use std::str::FromStr;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        // 生成一个创建缺省值的闭包,这里主要是为了简化后续代码
        // Default::default() 返回的类型根据上下文能够推导出来是Self
        // 而我们约定了Self,也就是T需要实现Default trait
        let d = || T::default();
        if let Some(caps) = re.captures(s) {
            caps.get(0)
                .map_or(d(), |x| x.as_str().parse().unwrap_or(d()))
        } else {
            d()
        }
    }
}

//#[test]
fn parse_should_work1() {
    assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(u8::parse("abc"), 0);
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(f64::parse("1234abcd"), 1234.0);
    assert_eq!(f64::parse("1234.213abcd"), 1234.213);
}

fn main() {
    parse_should_work1();
    println!("Hello, world!");
}
