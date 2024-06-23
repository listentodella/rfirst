use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

#[allow(dead_code)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell",
        }
    }
}

fn print(v: impl Into<IpAddr>) {
    println!("{:?}", v.into())
}

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref())
}

fn main() {
    let v4 = "2.2.2.2".parse::<Ipv4Addr>().unwrap();
    let v6 = "::1".parse::<Ipv6Addr>().unwrap();

    // IpAddr 实现了 From<[u8; 4]>,转换ipv4地址
    print([1, 1, 1, 1]);
    // IpAddr 实现了 From<[u16; 8]>,转换ipv6地址
    print([0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);

    // IpAddr 实现了 From<Ipv4Addr>
    print(v4);
    // IpAddr 实现了 From<Ipv6Addr>
    print(v6);

    let lang = Language::Rust;

    print_ref("Hello, world!");
    print_ref("Hello, world!".to_string());
    // 我们自己定义的枚举类型也实现了 AsRef<str>
    print_ref(lang);
}
