use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

// 如果想要支持Hash, 可以用 #[derive(Hash)]
// 前提是每个字段都实现了Hash
// 如果要能作为HashMap的key, 则需要实现Eq, PartialEq
#[derive(Eq, PartialEq, Debug, Hash)]
struct Student<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Student<'a> {
    fn new(name: &'a str, age: u8) -> Self {
        Self { name, age }
    }
}

fn main() {
    let mut hasher = DefaultHasher::new();
    let student = Student::new("Alice", 20);

    // 实现了 Hash 的数据结构可以直接调用 hash 方法
    student.hash(&mut hasher);

    let mut map = HashMap::new();
    // 实现了Hash/Eq/PartialEq的数据结构可以直接作为HashMap的key
    map.insert(student, vec!["Math", "Writing"]);
    println!("hash: 0x{:x}, map: {:?}", hasher.finish(), map);
}
