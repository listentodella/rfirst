use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Identifier<T> {
    inner: u64,
    // 如果不加 PhantomData<T>,并且该结构体的其他地方也没用到T,编译器不会允许通过
    _tag: PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct User {
    id: Identifier<Self>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Product {
    id: Identifier<Self>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_should_not_be_the_same() {
        let user = User::default();
        let product = Product::default();
        //两个id不能比较,因为它们属于不同的类型
        //一个是Identifier<User>,另一个是Identifier<Product>
        //assert_ne!(user.id, product.id);

        //但是它们的inner值是同类型的,可以比较
        assert_eq!(user.id.inner, product.id.inner);
    }
}
