use anyhow::{Ok, Result};
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, ReadBuf},
};

#[pin_project]
struct FileWrapper {
    #[pin]
    file: File,
}

impl FileWrapper {
    pub async fn try_new(name: &str) -> Result<Self> {
        let file = File::open(name).await?;
        Ok(Self { file })
    }
}

// 为自定义的数据结构实现 Async IO 之 tokio::AsyncRead
impl AsyncRead for FileWrapper {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        //不过这里的file是tokio的file,所以可以取巧使用自带的poll_read
        self.project().file.poll_read(cx, buf)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut file = FileWrapper::try_new("Cargo.toml").await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    println!("{:?}", buffer);

    Ok(())
}
