use anyhow::Result;
use serde_yaml::Value;
use std::fs;

fn toml2yaml(content: &str) -> Result<String> {
    let val: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&val)?)
}

fn main() -> Result<()> {
    // 读取cargo.toml文件, io 操作1
    let content1 = fs::read_to_string("./Cargo.toml").expect("failed to read file Cargo.toml");
    // 读取cargo.lock文件, io 操作2
    let content2 = fs::read_to_string("./Cargo.lock").expect("failed to read file Cargo.lock");

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;
    println!("{}\n{}", yaml1, yaml2);

    // 写入 ./Cargo.yaml 文件, io 操作3
    let _ = fs::write("./Cargo.toml.yaml", yaml1).expect("Failed to write to file Cargo.toml.yaml");
    // 写入 ./Cargo.lock 文件, io 操作4
    let _ = fs::write("./Cargo.lock.yaml", yaml2).expect("Failed to write to file Cargo.lock.yaml");

    Ok(())
}
