#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

const char *hello_world();

/// # Safety
/// 这个函数其实是不安全的
const char *hello_bad(const char *name);

const char *hello(const char *name);

/// # Safety
/// 提供给C侧 释放字符串指针,调用者自己需要保证指针来自rust
void free_str(char *ptr);

} // extern "C"
