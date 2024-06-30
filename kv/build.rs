use std::process::Command;

fn main() {
    Command::new("mkdir")
        .args(&["-p", "src/pb"])
        .status()
        .expect("mkdir failed!");

    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    // 该配置项会为所有类型添加 PartialOrd trait
    // 小心如果编译生成的如果自带了重复的属性, 使用它时可能导致编译失败
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
    Command::new("cargo")
        .args(&["fmt", "--", "src/*.rs"])
        .status()
        .expect("cargo fmt failed");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=abi.proto");
}
