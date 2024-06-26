use crate::{
    command_request::RequestData, CommandRequest, CommandResponse, KvError, MemTable, Storage,
};
use std::sync::Arc;
use std::thread;
use tracing::debug;

mod command_service;

/// 对 `Command` 的处理的抽象
pub trait CommandService {
    /// 处理 `Command`，返回 `CommandResponse`
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

/// `Service` 数据结构
pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store> Service<Store> {
    pub fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// `ServiceInner` 结构
struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        // TODO: 发送 on_received 事件
        let rsp = dispatch(cmd, &self.inner.store);

        debug!("Executed response: {:?}", rsp);
        // TODO: 发送 on_executed 事件

        rsp
    }
}

// 从 `Request` 中得到的 `Response`, 目前处理 `HSET`/`HGET`/`HGETALL`命令
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hset(v)) => v.execute(store),
        Some(RequestData::Hget(v)) => v.execute(store),
        Some(RequestData::Hgetall(v)) => v.execute(store),
        None => KvError::InvalidCommand("Request has no data".into()).into(),
        _ => KvError::Internal("Not implemented".into()).into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MemTable, Value};

    #[test]
    fn service_should_works() {
        // 我们需要一个 service 结构至少包含 Storage
        let service = Service::new(MemTable::default());

        // service 可以运行在多线程环境下, 它的clone 应该是轻量级的
        let cloned = service.clone();

        // 创建一个线程, 在 table t1 中写入 k1, v1
        let handle = thread::spawn(move || {
            let rsp = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_rsp_ok(rsp, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        // 在当前线程下读取 table t1 中的 k1, 应该返回 v1
        let rsp = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_rsp_ok(rsp, &["v1".into()], &[]);
    }
}

#[cfg(test)]
use crate::{Kvpair, Value};

#[cfg(test)]
// 测试成功返回的结果
pub fn assert_rsp_ok(mut rsp: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    rsp.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(rsp.status, 200);
    assert_eq!(rsp.message, "");
    assert_eq!(rsp.pairs, pairs);
    assert_eq!(rsp.values, values);
}

#[cfg(test)]
// 测试失败返回的结果
pub fn assert_rsp_err(rsp: CommandResponse, code: u32, msg: &str) {
    assert_eq!(rsp.status, code);
    assert!(rsp.message.contains(msg));
    assert_eq!(rsp.pairs, &[]);
    assert_eq!(rsp.values, &[]);
}
