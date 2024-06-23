use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imagine: self.imagine + rhs.imagine,
        }
    }
}

// 这样就不会移动所有权了
impl Add for &Complex {
    // 注意这里不能是 Self，因为在这个上下文中, Self 是 &Complex
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imagine: self.imagine + rhs.imagine,
        }
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real + rhs,
            imagine: self.imagine,
        }
    }
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(2 as f64, 4.0);
    let c = a + b;
    println!("{:?}", c);
    // 这里会报错，因为 a 和 b 已经被移动了
    // println!("{:?}", a);
    // println!("{:?}", b);

    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(2 as f64, 4.0);
    let d = &c1 + &c2;
    println!("{:?}", d);
    println!("{:?}", c1);
    println!("{:?}", c2);

    println!("{:?}", &c1 + 3.0);
}
