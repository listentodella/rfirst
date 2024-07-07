// 生成的 bindings 代码根据c/c++代码生成
// 里面有一些不符合rust约定, 我们不让编译器报警
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use anyhow::{anyhow, Result};
use std::mem;

mod bindings;
pub use bindings::*;
