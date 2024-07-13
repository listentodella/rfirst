use anyhow::Result;
use futures::prelude::*;
use tokio::{fs::File, io::AsyncWriteExt};

/// 使用unfold生成一个Sink数据结构
/// 并且通过unfold方法,不需要再实现Sink的几个方法了
fn writer<'a>(file: File) -> impl Sink<&'a str> {
    sink::unfold(file, |mut file, line: &'a str| async move {
        file.write_all(line.as_bytes()).await?;
        eprint!("Received: {}", line);
        Ok::<_, std::io::Error>(file)
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_sink = writer(File::create("/tmp/hello").await?);

    // pin_mut 可以把变量pin住
    futures::pin_mut!(file_sink);
    if file_sink.send("hello\n").await.is_err() {
        println!("Error on send");
    }
    if file_sink.send("world\n").await.is_err() {
        println!("Error on send");
    }

    Ok(())
}
