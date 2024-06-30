use crate::*;

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}
impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_request::RequestData;

    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let rsp = dispatch(cmd, &store);
        assert_rsp_ok(rsp, &[Value::default()], &[]);
    }

    #[test]
    fn hget_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        dispatch(cmd.clone(), &store);
        let cmd = CommandRequest::new_hget("t1", "hello");
        let rsp = dispatch(cmd, &store);
        assert_rsp_ok(rsp, &["world".into()], &[]);
    }

    #[test]
    fn hget_with_non_exist_key_should_return_404() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hget("score", "u1");
        let rsp = dispatch(cmd, &store);
        assert_rsp_err(rsp, 404, "Not found");
    }

    #[test]
    fn hgetall_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("score", "u1", 1.into()),
            CommandRequest::new_hset("score", "u2", 2.into()),
            CommandRequest::new_hset("score", "u3", 3.into()),
            CommandRequest::new_hset("score", "u1", 4.into()),
        ];

        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd = CommandRequest::new_hgetall("score");
        let rsp = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("u1", 4.into()),
            Kvpair::new("u2", 2.into()),
            Kvpair::new("u3", 3.into()),
        ];
        assert_rsp_ok(rsp, &[], pairs);
    }
}
