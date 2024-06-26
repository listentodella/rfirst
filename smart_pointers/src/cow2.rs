use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}

fn main() {
    let input = r#"{"name": "Alice", "age": 30}"#;
    let user: User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(name) => println!("Borrowed: {}", name),
        Cow::Owned(name) => println!("Owned: {}", name),
    }
}
