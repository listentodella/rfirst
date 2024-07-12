use anyhow::Result;
use serde_yaml::Value;
use tokio::{fs, try_join};

#[tokio::main]
async fn main() -> Result<()> {
    //这里使用的是tokio::fs,它返回一个future,
    //然后使用 join/try_join用来轮询future
    //遇到阻塞就处理下一个,直到所有future产生结果
    //性能和线程版本几乎一致,但是消耗的资源其实要少很多(主要是无线程栈的消耗)

    // 如果不使用join/try_join,而是直接await的话,则和同步没有区别了
    // 因为await会运行future直到结束

    let f1 = fs::read_to_string("../Cargo.toml");
    let f2 = fs::read_to_string("../Cargo.lock");
    let (content1, content2) = try_join!(f1, f2)?;

    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    let f3 = fs::write("./Cargo.toml.yaml", &yaml1);
    let f4 = fs::write("./Cargo.lock.yaml", &yaml2);
    try_join!(f3, f4)?;

    println!("{yaml1}\n{yaml2}");

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}
