pub fn consume_iterator<F, Iter, T>(mut f: F)
where
    F: FnMut(i32) -> Iter, // F是一个闭包, 它接受一个i32参数, 返回一个Iter类型的值
    Iter: Iterator<Item = T>, // Iter 是一个Iterator, Item是T类型
    T: std::fmt::Debug,    // T实现了Debug trait
{
    // 由于f的类型F是返回iterator, 因此可以用for处理
    for item in f(10) {
        println!("{:?}", item); // item的类型是T, 因此可以用Debug trait打印
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_iterator() {
        consume_iterator(|x| (0..x).into_iter());
    }
}
