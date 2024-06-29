use std::panic;

fn main() {
    let ret = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(ret.is_ok());

    let ret = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(ret.is_err());

    println!("panic captured: {:#?}", ret);
}
