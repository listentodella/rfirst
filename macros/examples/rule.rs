macro_rules! my_vec {
    // 如果没有任何参数, 创建一个空的vec
    () => {
        std::vec::Vec::new()

    };
    // 处理形如 my_vec![1,2,3]的情况
    ($($el:expr), *) => (
        {
            let mut v = std::vec::Vec::new();
            $(v.push($el);)*
            v
        }
    );
    // 处理形如 my_vec![1; 3]的情况
    ($el:expr; $n:expr) => {
        std::vec::from_elem($el, $n)
    }
}

fn main() {
    let mut v = my_vec![];
    v.push(1);
    // 调用时可以使用[], {}, ()
    let _v = my_vec!(1, 2, 3);
    let _v = my_vec![1, 2, 3];
    let v = my_vec! {1, 2, 3};
    println!("{:?}", v);
    let v = my_vec![1;3];
    println!("{:?}", v);
}
