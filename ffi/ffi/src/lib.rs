use libc::c_char;

use std::{
    ffi::{CStr, CString},
    panic::catch_unwind,
    ptr,
};

// 通过no_mangle属性，告诉编译器不要对此函数进行名称修饰，以便在C语言中调用
#[no_mangle]
pub extern "C" fn hello_world() -> *const c_char {
    // C String 以"\0"结尾,你可以把"\0"去掉看看会发生什么
    "hello world!\0".as_ptr() as *const c_char
}

/// # Safety
/// 这个函数其实是不安全的
#[allow(dead_code)]
#[no_mangle]
pub unsafe extern "C" fn hello_bad(name: *const c_char) -> *const c_char {
    //unsafe 1: 没有检查是否为NULL, 是否是合法地址
    //unsafe 2: unwrap()会导致stack unwind, 它会跨越FFI边界, 导致未定义行为
    let s = CStr::from_ptr(name).to_str().unwrap();

    //format!虽然生成了一个String, 但是 as_ptr()取到它堆上的其实位置
    //但是整个函数执行结束时,s退出作用域,它的堆内存会被drop掉
    format!("hello {}!\0", s).as_ptr() as *const c_char
}

// 编译器会报警 str / String 不是FFI
// #[no_mangle]
// pub extern "C" fn goodbye(name: &str) -> String {
//     format!("goodbye {}!", name)
// }

#[no_mangle]
pub extern "C" fn hello(name: *const c_char) -> *const c_char {
    if name.is_null() {
        return ptr::null();
    }

    let ret = catch_unwind(|| {
        if let Ok(s) = unsafe { CStr::from_ptr(name).to_str() } {
            let ret = format!("hello {}!", s);
            // 可以使用unwrap(),因为ret不包含 \0
            let s = CString::new(ret).unwrap();
            // 将一个CString转换为c的string,并将所有权交付给c
            // c释放时尽量不要直接使用free,而应该使用CString::from_raw()
            s.into_raw()
            // 相当于:
            // let p = s.as_ptr();
            // std::mem::forget(s);
            // p
        } else {
            ptr::null()
        }
    });

    match ret {
        Ok(ptr) => ptr,
        Err(_) => ptr::null(),
    }
}

/// # Safety
/// 提供给C侧 释放字符串指针,调用者自己需要保证指针来自rust
#[no_mangle]
pub unsafe extern "C" fn free_str(ptr: *mut c_char) {
    if !ptr.is_null() {
        // 巧妙地将指针传递给rust,释放,交由rust所有权机制决定
        let _ = CString::from_raw(ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_works() {
        let cstr = hello_world();
        let s = unsafe { CStr::from_ptr(cstr).to_str().unwrap() };
        assert_eq!(s, "hello world!");
    }

    #[test]
    fn hello_works() {
        let name = CStr::from_bytes_with_nul(b"Leo\0").unwrap().as_ptr();
        let cstr = hello(name);
        println!("{:p}", cstr);
        let s = unsafe { CStr::from_ptr(cstr).to_str().unwrap() };
        assert_eq!(s, "hello Leo!");
        // rust 分配的内存, rust 释放
        // 其实cast *const 为 *mut 是一个不好的习惯
        // 这里只是演示 c/cpp 如何把指针回传,让rust来释放内存
        unsafe { free_str(cstr as *mut c_char) };
    }
}
