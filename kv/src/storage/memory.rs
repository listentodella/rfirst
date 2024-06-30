use crate::{KvError, Kvpair, Storage, Value};
use dashmap::{mapref::one::Ref, DashMap};

/// 使用 `DashMap` 构建的 `MemTable` 结构, 实现了 `Storage` trait.
#[derive(Clone, Debug, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    /// 创建一个默认的 `MemTable` 实例.
    pub fn new() -> Self {
        Self::default()
    }

    /// 如果名为 `name` 的 `table` 不存在, 则创建一个新的 `table` 并返回.
    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => self.tables.entry(name.into()).or_default().downgrade(),
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.get(key).map(|v| v.clone()))
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.insert(key, value))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        Ok(self.get_or_create_table(table).contains_key(key))
    }
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        Ok(self.get_or_create_table(table).remove(key).map(|(_k, v)| v))
    }
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        Ok(self
            .get_or_create_table(table)
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect())
    }
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        todo!()
    }
}
