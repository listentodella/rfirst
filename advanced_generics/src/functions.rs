pub trait ImplTrait {
    // 允许
    fn impl_in_args(s: impl Into<String>) -> String {
        s.into()
    }

    // 虽然对于所有分支的返回值,如果它们类型相同,则允许,但不推荐
    // 因为如果返回两种不同的类型,即便它们都实现了 Into<String>
    // 但对于目前的编译器而言,返回值属于不同的类型,导致类型不匹配
    fn impl_as_return(s: String) -> impl Into<String> {
        // if s.len() > 5 {
        //     s.as_bytes()
        // } else {
        //     s
        // }
        s
    }
}

// 可以正确编译
pub fn generics_as_return_working(i: u32) -> impl Iterator<Item = u32> {
    std::iter::once(i)
}

// 期待泛型参数,却返回一个具体类型
// pub fn generics_as_return_not_working<T: Iterator<Item = u32>>(i: u32) -> T {
//     std::iter::once(i)
// }

// 返回trait object
pub fn trait_object_as_return_working(i: u32) -> Box<dyn Iterator<Item = u32>> {
    Box::new(std::iter::once(i))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_return_impl() {
        let mut iter = generics_as_return_working(10);
        assert_eq!(iter.next(), Some(10));
    }

    #[test]
    fn test_return_trait_object() {
        let mut iter = trait_object_as_return_working(10);
        assert_eq!(iter.next(), Some(10));
    }
}
