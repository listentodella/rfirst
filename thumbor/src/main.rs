//某种处理方式
enum Spec {
    Resize(Resize),
    Crop(Crop),
}

//解析出来的图片处理参数
struct ImageSpec {
    specs: Vec<Spec>,
}

struct Resize {
    width: u32,
    height: u32,
}

struct Crop {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");
}
