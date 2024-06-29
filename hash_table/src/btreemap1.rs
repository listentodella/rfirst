use std::collections::BTreeMap;

// BTreeMap 结构有 height, node 和 length
// 由于该函数获取了所有权, 但我们之后还想使用 map, 所以要 transmute 回原来的类型
fn explain<K, V>(action: &str, map: BTreeMap<K, V>) -> BTreeMap<K, V> {
    let arr: [usize; 3] = unsafe { std::mem::transmute(map) };

    println!(
        "{}: height: {}, root node: 0x{:x}, len: 0x{:x}",
        action, arr[0], arr[1], arr[2]
    );

    unsafe { std::mem::transmute(arr) }
}

fn main() {
    let map = BTreeMap::new();
    let mut map = explain("empty", map);

    for i in 0..16usize {
        map.insert(format!("Leo {}", i), i);
    }

    let mut map = explain("16 elements", map);

    map.remove("Leo 1");

    map = explain("remove Leo 1", map);

    // BTreeMap 根据 Key 排序, 所以遍历时会按照 Key 排序
    // 即 BTreeMap是有序的
    for item in map.iter() {
        println!("{:?}", item);
    }
}
