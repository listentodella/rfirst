use std::collections::BTreeMap;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Hash)]
struct Name {
    pub name: String,
    pub flags: u32,
}

impl Name {
    pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            flags,
        }
    }
}

fn main() {
    let mut map = BTreeMap::new();
    map.insert(Name::new("/etc/password", 0x1), 12);
    map.insert(Name::new("/etc/hosts", 0x2), 4);
    map.insert(Name::new("/home/Leo", 0x0), 28);

    for i in map.iter() {
        println!("{:?}", i);
    }
}
