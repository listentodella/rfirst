use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+").unwrap();
        if let Some(caps) = re.captures(s) {
            caps.get(0).map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

impl Parse for f64 {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?").unwrap();
        if let Some(caps) = re.captures(s) {
            caps.get(0)
                .map_or(0.0, |s| s.as_str().parse().unwrap_or(0.0))
        } else {
            0.0
        }
    }
}

#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(u8::parse("abc"), 0);
    assert_eq!(u8::parse("1234abcd"), 0);
}

fn main() {
    println!("ret: {}", u8::parse("255 hello world"));
    println!("ret: {}", f64::parse("255.32321 hello world"));
}
