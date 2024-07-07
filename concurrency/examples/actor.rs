/// actor 是一种有栈协程。
/// 每个 actor，有自己的一个独立的、轻量级的调用栈
/// 以及一个用来接受消息的消息队列（mailbox 或者 message queue）
/// 外界跟 actor 打交道的唯一手段就是，给它发送消息
use actix::prelude::*;
use anyhow::Result;

// actor 可以处理的消息
#[derive(Message, Debug, Clone, PartialEq)]
#[rtype(result = "OutMsg")]
enum InMsg {
    Add((usize, usize)),
    Concat((String, String)),
}

#[derive(MessageResponse, Debug, Clone, PartialEq)]
enum OutMsg {
    Num(usize),
    Str(String),
}

// Actor
struct DummyActor;

impl Actor for DummyActor {
    type Context = Context<Self>;
}

// 实现处理 InMsg 的 Handler trait
impl Handler<InMsg> for DummyActor {
    type Result = OutMsg; //<- 返回的消息

    fn handle(&mut self, msg: InMsg, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            InMsg::Add((a, b)) => OutMsg::Num(a + b),
            InMsg::Concat((mut s1, s2)) => {
                s1.push_str(&s2);
                OutMsg::Str(s1)
            }
        }
    }
}

#[actix::main]
async fn main() -> Result<()> {
    let addr = DummyActor.start();
    let ret = addr.send(InMsg::Add((21, 21))).await?;
    let ret1 = addr
        .send(InMsg::Concat(("hello, ".into(), "world!".into())))
        .await?;

    println!("ret:{:?}, ret1:{:?}", ret, ret1);
    Ok(())
}
