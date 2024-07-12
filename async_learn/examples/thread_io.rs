use anyhow::{anyhow, Result};
use serde_yaml::Value;
use std::{
    fs,
    thread::{self, JoinHandle},
};

/// 包装一下 JoinHandle，这样可以提供额外的方法
struct MyJoinHandle<T>(JoinHandle<Result<T>>);

impl<T> MyJoinHandle<T> {
    pub fn thread_await(self) -> Result<T> {
        self.0.join().map_err(|_| anyhow!("failed"))?
    }
}

fn main() -> Result<()> {
    //读取Cargo.toml文件,io操作1
    let t1 = thread_read("../Cargo.toml");
    //读取Cargo.lock文件,io操作2
    let t2 = thread_read("../Cargo.lock");

    let content1 = t1.thread_await()?;
    let content2 = t2.thread_await()?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入, io操作3，4
    let t3 = thread_write("./Cargo.toml.yaml", yaml1);
    let t4 = thread_write("./Cargo.lock.yaml", yaml2);

    let yaml1 = t3.thread_await()?;
    let yaml2 = t4.thread_await()?;

    fs::write("./Cargo.toml.yaml", &yaml1)?;
    fs::write("./Cargo.lock.yaml", &yaml2)?;

    // 打印
    println!("Cargo.toml.yaml:\n{}", &yaml1);
    println!("Cargo.lock.yaml:\n{}", &yaml2);

    Ok(())
}

fn thread_read(filename: &'static str) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        let s = fs::read_to_string(filename)
            .expect(format!("failed to read file {}", filename).as_str());
        Ok::<_, anyhow::Error>(s)
    });
    MyJoinHandle(handle)
}

fn thread_write(filename: &'static str, content: String) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        fs::write(filename, &content).expect(format!("failed to write file {}", filename).as_str());
        Ok::<_, anyhow::Error>(content)
    });
    MyJoinHandle(handle)
}

fn toml2yaml(content: &str) -> Result<String> {
    println!("toml2yaml: {}", content);
    let value: Value =
        toml::from_str(content).expect(format!("failed to from str to yaml").as_str());
    Ok(serde_yaml::to_string(&value)?)
}
