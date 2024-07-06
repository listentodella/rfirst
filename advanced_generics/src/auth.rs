use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 从后续的代码可以看到, _type并没有被使用
/// 但我们确实需要将Customer以泛型的方式定义,以便区分不同的用户类型
/// 但是如果不定义这么个PhantomData,编译器会报错
pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("Feature 1 for customer {}", self.name);
    }
    fn feature2(&self) {
        println!("Feature 2 for customer {}", self.name);
    }
}

pub struct FreePlan;
pub struct PersonalPlan(f32);

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!(
            "Dear {}(as VIP {}), enjoy advanced feature!",
            self.name, self.id
        );
    }
}

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            //尽管_type只是个PhantomData,没有大小,但是依旧需要给它初始化
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(value: Customer<FreePlan>) -> Self {
        Self::new(value.name)
    }
}

/// 订阅成为付费用户
pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    // do something here

    customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_customer() {
        // 一开始是个免费用户
        let customer = Customer::<FreePlan>::new("Leo".to_string());
        // 使用免费的feature
        customer.feature1();
        customer.feature2();
        // 升级为付费用户
        let customer = subscribe(customer, 648.0);
        // 可以继续使用免费的feature
        customer.feature1();
        customer.feature2();
        // 也可以使用付费的feature
        customer.advance_feature();
    }
}
