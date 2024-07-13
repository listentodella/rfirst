`Stream`和`Iterator`类似:  
```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    fn size_hint(&self) -> (usize, Option<usize>) { ... }
    fn map<B, F>(self, f: F) -> Map<Self, F> where F: FnMut(Self::Item) -> B { ... }
    ... // 还有 67 个方法
}

pub trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>,  cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;

    fn size_hint(&self) -> (usize, Option<usize>) { ... }
}

pub trait StreamExt: Stream {
    fn next(&mut self) -> Next<'_, Self> where Self: Unpin { ... }
    fn map<T, F>(self, f: F) -> Map<Self, F> where F: FnMut(Self::Item) -> T { ... }
    ... // 还有 41 个方法
}
```

由于 `poll_next()`使用起来不那么方便,需要调用者自己处理Poll状态, `StreamExt` 提供了`next()`,这样开发者就可以直接使用`stream.next().await`来获取下一个值了:
```rust
pub trait StreamExt: Stream {
    fn next(&mut self) -> Next<'_, Self> where Self: Unpin {
        assert_future::<Option<Self::Item>, _>(Next::new(self))
    }
}

// next 返回了 Next 结构
pub struct Next<'a, St: ?Sized> {
    stream: &'a mut St,
}

// 如果 Stream Unpin 那么 Next 也是 Unpin
impl<St: ?Sized + Unpin> Unpin for Next<'_, St> {}

impl<'a, St: ?Sized + Stream + Unpin> Next<'a, St> {
    pub(super) fn new(stream: &'a mut St) -> Self {
        Self { stream }
    }
}

// Next 实现了 Future，每次 poll() 实际上就是从 stream 中 poll_next()
impl<St: ?Sized + Stream + Unpin> Future for Next<'_, St> {
    type Output = Option<St::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.stream.poll_next_unpin(cx)
    }
}
```


