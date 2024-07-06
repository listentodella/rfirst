# Unsafe Rust的使用场景

## 1. 与硬件交互
例如操作IO访问外设,或者使用汇编指令进行特殊操作

## 2. 外部语言
比如调用C、C++的库,即FFI(Foreign Function Interface)

## 3. 性能优化
比如略过边界检查、使用未初始化内存等,不过并不建议!