use bindgen::CargoCallbacks;

fn main() {
    // build.rs 可以通过 `println!` 的输出内容,与cargo进行通信
    // cargo会将每一行 `cargo:`前缀的输出解析为一条指令,其他的输出内容则会被忽略

    // 告诉rustc需要link bzip2
    println!("cargo:rustc-link-lib=bz2");

    // 告诉cargo当wrapper.h发生变化时,需要重新编译
    println!("cargo:rerun-if-changed=wrapper.h");

    // 配置bindgen, 并生成 Bindings 结构
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings");

    // 生成 rust 代码
    bindings
        .write_to_file("src/bindings.rs")
        .expect("failed to write bindings");
}
