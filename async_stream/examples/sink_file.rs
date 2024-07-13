use anyhow::Result;
use bytes::{BufMut, BytesMut};
use futures::{Sink, SinkExt};
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::fs::File;
use tokio::io::AsyncWrite;

#[pin_project]
struct FileSink {
    #[pin]
    file: File,
    buf: BytesMut,
}

impl FileSink {
    pub fn new(file: File) -> Self {
        Self {
            file,
            buf: BytesMut::new(),
        }
    }
}

/// Sink trait的4个方法
/// - poll_ready()：用来准备 Sink 使其可以发送数据
///     只有 poll_ready() 返回 Poll::Ready(Ok(())) 后，Sink 才会开展后续的动作。
/// - poll_ready() 可以用来控制背压
/// - start_send()：开始发送数据到 Sink
///     但是 start_send() 并不保证数据被发送完毕，所以调用者要调用 poll_flush() 或者 poll_close() 来保证完整发送。
/// - poll_flush()：将任何尚未发送的数据 flush 到这个 Sink。
/// - poll_close()：将任何尚未发送的数据 flush 到这个 Sink，并关闭这个 Sink。
///
///
/// 其中3个方法与Item无关,这就意味着,如果有多个不同的输入类型,
/// Sink的poll_ready(), start_send(), poll_flush(), poll_close()的会单态化会有重复的代码.
/// 所以如果确实需要处理不同类型的输入,可以用enum封装它们,以缩小代码体积:)
///
///
/// tokio::fs::File已经实现了AsyncRead和AsyncWrite
/// 所以我们只需实现剩下的3个方法即可
impl Sink<&str> for FileSink {
    type Error = std::io::Error;
    fn poll_ready(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: &str) -> std::result::Result<(), Self::Error> {
        let this = self.project();
        eprint!("{}", item);
        this.buf.put(item.as_bytes());
        Ok(())
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        //如果想project()多次,需要先把self reborrow一下
        let this = self.as_mut().project();
        // 对已有的内容调用split_to(),得到一个包含所有未写入文件的新buffer
        // 这个buffer和self无关, 所以下面传入到poll_write()时,不会有对self的引用问题
        let buf = this.buf.split_to(this.buf.len());
        if buf.is_empty() {
            return Poll::Ready(Ok(()));
        }
        //写入文件
        if let Err(e) = futures::ready!(this.file.poll_write(cx, &buf[..])) {
            return Poll::Ready(Err(e));
        }
        //刷新文件
        self.project().file.poll_flush(cx)
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        let this = self.project();
        //结束写入
        this.file.poll_shutdown(cx)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_sink = FileSink::new(File::create("/tmp/hello").await?);
    //pin_mut可以把变量pin住
    futures::pin_mut!(file_sink);
    file_sink.send("hello\n").await?;
    file_sink.send("world\n").await?;
    file_sink.send("Leo\n").await?;

    Ok(())
}
