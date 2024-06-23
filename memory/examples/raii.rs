use std::fs::File;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
// 尽管我们没有显式调用close,但rust可以知道file的生命周期
// 然后帮我们释放掉
// 这是GC都做不到的,因为gc只对内存负责,其他资源依旧要靠开发者自己
