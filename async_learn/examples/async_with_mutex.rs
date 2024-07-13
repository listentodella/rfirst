use anyhow::Result;
use std::{sync::Arc, time::Duration};
// 使用tokio的Mutex而非标准库的Mutex
use tokio::sync::Mutex;

struct DB;

impl DB {
    //模拟commit数据
    async fn commit(&mut self) -> Result<usize> {
        Ok(42)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db1 = Arc::new(Mutex::new(DB));
    let db2 = db1.clone();

    tokio::spawn(async move {
        let mut db = db1.lock().await;
        //因为拿到的MutexGuard要跨越await,所以不能用std::sync::Mutex
        //只能用tokio::sync::Mutex
        let affected = db.commit().await?;
        println!("db1:Total affected rows:{}", affected);
        Ok::<_, anyhow::Error>(())
    });

    tokio::spawn(async move {
        let mut db = db2.lock().await;
        let affected = db.commit().await?;
        println!("db2:Total affected rows:{}", affected);
        Ok::<_, anyhow::Error>(())
    });

    //让两个task都有机会执行完成

    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
