fn main() {
    let ret = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|x| x * x)
        .filter(|y| *y > 16)
        .take(1)
        .collect::<Vec<_>>();

    println!("{:?}", ret);
}
