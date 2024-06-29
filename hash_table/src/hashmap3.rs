use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.insert("d", 4);

    // 每次打印结果都不同
    // 所以HashMap的迭代顺序是不确定的
    // 即HashMap是无序的
    for i in map.iter() {
        println!("{:?}", i);
    }
}
