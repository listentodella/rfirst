use std::{marker::PhantomPinned, pin::Pin, ptr::null};

#[derive(Debug)]
struct SelfReference {
    name: String,
    name_ptr: *const String,
    // PhantomPinned 占位符
    _marker: PhantomPinned,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        SelfReference {
            name: name.into(),
            name_ptr: null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        let name_ptr = &self.name as *const String;
        // SAFETY: 这里并不会把任何数据从 &mut SelfReference中移走
        let this = unsafe { self.get_unchecked_mut() };
        this.name_ptr = name_ptr;
    }

    pub fn print_name(self: Pin<&Self>) {
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
    move_creates_issue();
}

fn move_creates_issue() {
    let mut data = SelfReference::new("Leo");
    // 这里将data封装进Pin
    let mut data = unsafe { Pin::new_unchecked(&mut data) };
    // as_mut() 返回Pin<&mut SelfReference>
    SelfReference::init(data.as_mut());

    // 不move,一切ok
    data.as_ref().print_name();

    // 现在只能拿到pinne后的数据
    move_pinned(data.as_mut());
    println!("{:?} ({:p})", data, &data);

    // 你无法拿回Pin之前的SelfReference结构,所以调用不了move_it
    //move_it(data);
}

// 数据结构被包裹在Pin内部,所以在函数间传递时,变化的只是指向data的Pin
// 而data本身不会有变化
fn move_pinned(data: Pin<&mut SelfReference>) {
    println!("{:?} ({:p})", data, &data);
}

#[allow(dead_code)]
fn move_it(data: SelfReference) {
    println!("{:?} ({:p})", data, &data);
}
