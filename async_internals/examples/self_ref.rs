#[derive(Debug)]
struct SelfReference {
    name: String,
    // 在初始化后让它指向name
    name_ptr: *const String,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        SelfReference {
            name: name.into(),
            name_ptr: std::ptr::null(),
        }
    }

    pub fn init(&mut self) {
        self.name_ptr = &self.name as *const String;
    }

    pub fn print_name(&self) {
        println!(
            "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
            self,
            &self.name,
            self.name_ptr,
            self.name,
            unsafe { &*self.name_ptr }
        );
    }
}

fn main() {
    let data = move_creates_issue();
    println!("data:{:?}", data);
    // 如果注释掉下面这句,程序会segment fault
    // data.print_name();
    println!();
    mem_swap_creates_issue();
}

fn move_creates_issue() -> SelfReference {
    let mut data = SelfReference::new("Leo");
    data.init();
    data.print_name();

    let data = move_it(data);
    //move之后,name_ref指向的位置已经是失效的地址
    //如果这里没出问题,也只是因为指向的地址还没被其他程序使用
    data.print_name();
    data
}

fn mem_swap_creates_issue() {
    let mut data1 = SelfReference::new("Leo");
    data1.init();

    let mut data2 = SelfReference::new("Bob");
    data2.init();

    data1.print_name();
    data2.print_name();

    std::mem::swap(&mut data1, &mut data2);
    data1.print_name();
    data2.print_name();
}

fn move_it(data: SelfReference) -> SelfReference {
    data
}
