use anyhow::Result;
use std::time::Duration;

// 强制tokio只使用一个工作线程,这样task2不会跑到其他线程执行
#[tokio::main(worker_threads = 1)]
async fn main() -> Result<()> {
    // 先开始执行的task1如果一直不让出cpu,则task2永远不会被执行
    let task1 = tokio::spawn(async move {
        eprintln!("task1");
        // yield 或 sleep等操作会让出cpu
        // tokio::time::sleep(Duration::from_millis(1)).await;
        // tokio::task::yield_now().await;
        // eprintln!("yield or sleep done");
        loop {}
    });

    let task2 = tokio::spawn(async move {
        eprintln!("task2");
    });

    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}
