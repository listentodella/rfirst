use std::alloc::{GlobalAlloc, Layout, System};

// 在Mac上无法运行...
struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data = System.alloc(layout);
        eprint!("ALLOC: {:p}, size: {}\n", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        eprint!("FREE/DEALLOC: {:p}, size: {}\n", ptr, layout.size())
    }
}

// 该宏用于声明全局分配器，并将其设置为 MyAllocator
#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

#[allow(dead_code)]
struct Matrix {
    //使用不规则的数字,例如505,可以让dbg!输出的结果更加直观
    data: [u8; 505],
}

impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 505] }
    }
}

fn main() {
    // 在这句之前已经有许多内存分配
    let data = Box::new(Matrix::default());

    // 输出中有一个1024大小的内存分配,是println!导致
    println!(
        "!!! allocated memory: {:p}, len: {}",
        &*data,
        std::mem::size_of::<Matrix>()
    );

    // data 在这里drop,可以看到FREE
    // 之后还有很多其他内存会被释放
}
