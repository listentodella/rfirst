mod memory;
pub use memory::MemTable;

use crate::{KvError, Kvpair, Value};

/// 对存储的抽象, 我们不关心数据存在哪儿, 但需要定义外界如何与存储打交道
pub trait Storage {
    /// 从一个 `HashTable` 里获取一个 `key` 的 `value`
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个 `HashTable` 里设置一个 `key` 的 `value`, 返回旧的 `value` (如果有的话)
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看 `HashTable` 中是否有 `key`
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 从一个 `HashTable` 里删除一个 `key`
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 遍历 `HashTable`, 返回所有的 `kv pair`(这个接口不好)
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// 遍历 `HashTable`, 返回 `kv pair` 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_basic_interface(store: impl Storage) {
        // 第一次 set 会创建table, 插入key并返回None(因为之前没有)
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());

        // 再次 set 同样的 key 会更新value并返回旧的value
        let v1 = store.set("t1", "hello".into(), "world1".into());
        assert_eq!(v1.unwrap().unwrap(), "world".into());

        // get 存在的key会得到最新的值
        let v = store.get("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        // get 不存在的key会得到None
        let v = store.get("t1", "hello1");
        assert_eq!(v, Ok(None));

        // contains 存在的key会得到true
        assert_eq!(store.contains("t1", "hello"), Ok(true));
        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));

        // del 存在的key 返回之前的值
        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        // del 不存在的key或table 会返回None
        assert_eq!(store.del("t1", "hello1"), Ok(None));
        assert_eq!(store.del("t2", "hello1"), Ok(None));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data = store.get_all("t2").unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        );
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t3", "k1".into(), "v1".into()).unwrap();
        store.set("t3", "k2".into(), "v2".into()).unwrap();
        let mut data: Vec<_> = store.get_iter("t3").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        );
    }

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    // #[test]
    // fn memtable_iter_should_work() {
    //     let store = MemTable::new();
    //     test_get_iter(store);
    // }
}
