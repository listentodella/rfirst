use core::{marker::PhantomData, ops};

struct MMIODerefWrapper<T> {
    // 被MMIO的设备的起始地址
    start_addr: usize,
    // 告诉编译器, 该泛型是一个函数指针, 其返回值类型为T
    phantom: PhantomData<fn() -> T>,
}

impl<T> MMIODerefWrapper<T> {
    /// 创建一个实例
    const unsafe fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // 先将数值转换为指针, 再通过*获取该地址的值
        // 再用& 表示获取值的不可变引用
        unsafe { &*(self.start_addr as *const _) }
    }
}

fn main() {
    let a = 255u8;
    let addr = &a as *const _ as usize;
    println!("get addr 0x{:x}", addr);
    unsafe {
        let r1 = MMIODerefWrapper::<u8>::new(addr);
        println!("get mmio r1 addr 0x{:x}", r1.start_addr);
        println!("get mmio r1 val 0x{:x}", *r1);
        println!("get mmio r1 val 0x{:x}", ops::Deref::deref(&r1));
    }
}
