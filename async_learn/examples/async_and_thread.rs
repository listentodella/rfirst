use anyhow::Result;
use blake3::Hasher;
use futures::{SinkExt, StreamExt};
use rayon::prelude::*;
use std::{any, thread};
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};
use tokio_util::codec::{Framed, LinesCodec};

pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on: {}", addr);

    //创建tokio task 和 thread 之间的channel
    let (sender, mut reciever) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

    // 使用thread处理计算密集型任务
    thread::spawn(move || {
        // 读取从tokio task 发过来msg
        // 注意这里使用的是 blocking_recv 而非 await
        while let Some((line, reply)) = reciever.blocking_recv() {
            // 计算pow
            let ret = match pow(&line) {
                Some((hash, nonce)) => format!("hash: {}, nonce: {}", hash, nonce),
x               None => "invalid input".to_string(),
            };
            // 把计算结果从oneshot channel 发回
            if let Err(e) = reply.send(ret) {
                println!("Failed to send result: {}", e);
            }
        }
    });

    // 使用tokio task 处理IO密集型任务
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);
        let sender1 = sender.clone();
        tokio::spawn(async move {
            // 使用LinesCodec把TCP数据切成一行行字符串处理
            let mut framed = Framed::new(stream, LinesCodec::new());
            // split 成 writer 和 reader
            let (mut w, mut r) = framed.split();
            while let Some(Ok(line)) = r.next().await {
                // 为每个消息创建一个oneshot channel,用于对端反馈回来
                let (reply, reply_receiver) = oneshot::channel();
                sender1.send((line, reply))?;

                if let Ok(v) = reply_receiver.await {
                    w.send(format!("Pow calc:{}", v)).await?;
                }
            }

            Ok::<_, anyhow::Error>(())
        });
    }
}

//------- 以下代码并不是本节的重点, 仅用于密集计算消耗cpu,确实很消耗,我的7950x也不能秒回---

// 使用 rayon 并发计算 u32 空间下所有 nonce，直到找到有头 N 个 0 的哈希
pub fn pow(s: &str) -> Option<(String, u32)> {
    let hasher = blake3_base_hash(s.as_bytes());
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });
    nonce.map(|n| {
        let hash = blake3_hash(hasher, &n).to_hex().to_string();
        (hash, n)
    })
}

// 计算携带 nonce 后的哈希
fn blake3_hash(mut hasher: blake3::Hasher, nonce: &u32) -> blake3::Hash {
    hasher.update(&nonce.to_be_bytes()[..]);
    hasher.finalize()
}

// 计算数据的哈希
fn blake3_base_hash(data: &[u8]) -> Hasher {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher
}
