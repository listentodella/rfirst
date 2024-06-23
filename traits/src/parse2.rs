use regex::Regex;
use std::str::FromStr;

pub trait Parse {
    type Err;
    fn parse(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    type Err = String;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(caps) = re.captures(s) {
            caps.get(0)
                .map_or(Err("failed to capture".to_string()), |x| {
                    x.as_str()
                        .parse()
                        .map_err(|_e| "failed to parse captured string".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

//#[test]
fn parse_should_work1() {
    assert_eq!(u8::parse("123abcd"), Ok(123));
    assert!(u8::parse("abc").is_err());
    assert_eq!(
        u8::parse("1234abcd"),
        Err("failed to parse captured string".to_string())
    );
    assert_eq!(f64::parse("1234abcd"), Ok(1234.0));
    assert_eq!(f64::parse("1234.213abcd"), Ok(1234.213));
}

fn main() {
    parse_should_work1();
    println!("Hello, world!");
}
