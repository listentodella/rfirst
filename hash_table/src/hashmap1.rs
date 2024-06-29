use std::collections::HashMap;

fn explain<K, V>(action: &str, map: &HashMap<K, V>) {
    println!(
        "{}: len = {}, capacity = {}",
        action,
        map.len(),
        map.capacity()
    );
}

fn main() {
    // 初始容量为0, 但增长时会以 2^N - 1 的方式增长
    let mut map = HashMap::new();
    explain("empty", &map);

    map.insert('a', 1);
    explain("after adding 1", &map);

    map.insert('b', 2);
    map.insert('c', 3);
    explain("after adding 3", &map);

    map.insert('d', 4);
    explain("after adding 4", &map);

    // get 时需要使用引用, 并且也会返回引用
    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

    map.remove(&'a');
    // 删除后就不存在了, 但是容量却不会变小
    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);
    explain("removed", &map);
    // 只有 shrink 后则会变小, 回收未使用的空间
    map.shrink_to_fit();
    explain("shrinked", &map);
}
