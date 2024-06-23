use std::fs::File;
use std::io::Write;

fn main() {
    let mut f = File::create("hello.txt").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"Hello").unwrap();
    //by_ref() 只能为 实现了Sized的对象使用
    //let w1 = w.by_ref();
    //w1.write_all(b"world!").unwrap();
}
