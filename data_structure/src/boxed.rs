use std::ops::Deref;

fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    v1.push(5);
    println!("cap should be 8, actual = {}", v1.capacity());

    // 从 Vec<T> 转换成Box<[T]>, 此时会丢弃多余的 capacity
    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b1.into_vec();
    println!("cap before boxed_slice 8, after = {}", v2.capacity());
    assert!(b2.deref() == v2);

    // Box<[T]> 可以更改其内部数据,但不能改变其容量,例如push
    b2[0] = 2;
    // b2.push(6);
    println!("b2 = {:?}", b2);

    //注意Box<[T]> 和 Box<[T;n]>并不相同
    let b3 = Box::new([2, 2, 3, 4, 5]);
    println!("b3 = {:?}", b3);
    // b2-Boxed<[i32]>和 b3-Boxed<[i32;5]> 实现了相互比较
    assert!(b2 == b3);
    // 但 b3.deref() 和 v2 无法比较
    // 因为 &[T;n] 和 Vec<T> 未实现相互比较的方法
    //assert!(b3.deref() == v2);
}
