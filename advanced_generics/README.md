# 泛型参数的常见使用场景  
## 使用泛型参数延迟数据结构的绑定
- 参考`BufReader`的实现, 并不是所有方法都是同样的限制, 有的甚至完全没有限制  

- 给泛型参数提供一个缺省值,好处是 在使用时,可以不提供泛型参数,直接使用缺省值. 这个泛型参数在随后的实现中可以逐渐约束

## 使用泛型参数和`PhantomData`,做“标记”,实际代码里并没有直接使用
比如:
```rust
struct Identifier<T> {
    inner:u64
}

struct User {
    id: Identifier<Self>,
}
struct Product{
    id: Identifier<Self>,
}


```

## 使用泛型参数,让同一个数据结构对同一个trait可以有不同的实现