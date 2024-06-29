use std::collections::HashMap;

// HashMap 结构有2个 u64 的 RandomState, 然后是4个usize
// 分别是 bucket_mask, ctrl, growth_left, items
// 我们transmute打印之后,再transmute回去
fn explain<K, V>(action: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };

    println!(
        "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
        action, arr[2], arr[3], arr[4], arr[5]
    );

    unsafe { std::mem::transmute(arr) }
}

fn main() {
    // 初始容量为0, 但增长时会以 2^N - 1 的方式增长
    let map = HashMap::new();
    let mut map = explain("empty", map);

    map.insert('a', 1);
    let mut map = explain("after adding 1", map);

    map.insert('b', 2);
    map.insert('c', 3);
    let mut map = explain("after adding 3", map);

    map.insert('d', 4);
    let mut map = explain("after adding 4", map);

    // get 时需要使用引用, 并且也会返回引用
    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

    map.remove(&'a');
    // 删除后就不存在了, 但是容量却不会变小
    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);
    let mut map = explain("removed", map);
    // 只有 shrink 后则会变小, 回收未使用的空间
    map.shrink_to_fit();
    let _ = explain("shrinked", map);
}
