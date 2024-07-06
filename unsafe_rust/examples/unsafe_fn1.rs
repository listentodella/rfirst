use std::collections::HashMap;

fn main() {
    let map = HashMap::new();
    let mut map = explain("empty", map);
    map.insert("a".to_string(), 1);
    explain("added 1", map);
}

// HashMap 结构有两个u64的RandomState
// 然后是4个usize: bucket_mask, ctrl, growth_left, items
// 我们 transmute 打印之后, 再 transmute 回去
fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
    println!("action:{name}, {:?}", arr);

    unsafe { std::mem::transmute(arr) }
}
