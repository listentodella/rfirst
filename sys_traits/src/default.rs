use std::{fmt, ops::Deref};

#[derive(Debug, Clone, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Language {
    Rust,
    Python,
    Java,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::Rust => write!(f, "Rust"),
            Language::Python => write!(f, "Python"),
            Language::Java => write!(f, "Java"),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({} years old): {} developer",
            self.name, self.age, self.lang
        )
    }
}

impl Developer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

fn main() {
    println!("Hello, world!");

    let dev1 = Developer::default();

    let dev2: Developer = Default::default();

    let dev3 = Developer::new("Leo");

    println!("dev1: {}", dev1);
    println!("dev2: {}", dev2);
    println!("dev3: {}", dev3);

    use std::sync::{Arc, Mutex};
    let shared = Arc::new(Mutex::new(1));
    // Arc 实现了 Deref<T>, 所以可以通过deref直接访问到里面的成员T
    // 而这里的T是Mutex,它有lock()方法,所以share看起来可以拥有lock()
    // 实际上是 deref() 让它间接访问到内部的T, 然后再调用lock()
    let mut g = shared.lock().unwrap();
    *g += 1;
}
