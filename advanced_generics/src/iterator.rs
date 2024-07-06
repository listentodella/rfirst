use std::marker::PhantomData;

#[derive(Debug, Default)]
// 这里的IterMethod是泛型参数,也可以单纯用T等表示
// 但是用一个词来表示,有时会更方便理解
pub struct Equation<IterMethod> {
    current: u32,
    _method: PhantomData<IterMethod>,
}

// 线性增长
#[derive(Debug, Default)]
pub struct Linear;

// 二次增长
#[derive(Debug, Default)]
pub struct Quadratic;

impl Iterator for Equation<Linear> {
    type Item = u32;

    //fn next(&mut self) -> Option<u32> {
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None;
        }
        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;

    //fn next(&mut self) -> Option<u32> {
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None;
        }
        Some(self.current * self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let mut equation = Equation::<Linear>::default();
        assert_eq!(equation.next(), Some(1));
        assert_eq!(equation.next(), Some(2));
        assert_eq!(equation.next(), Some(3));
    }
    #[test]
    fn test_quadratic() {
        let mut equation = Equation::<Quadratic>::default();
        assert_eq!(equation.next(), Some(1));
        assert_eq!(equation.next(), Some(4));
        assert_eq!(equation.next(), Some(9));
    }
}
