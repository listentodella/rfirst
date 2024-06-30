# 闭包的使用场景

## 作为返回值
当闭包作为一个函数的返回值时，调用者得到该返回值后，通过调用该返回值来执行闭包  

## 作为参数传递
当闭包作为参数传递给另一个函数时，该闭包会被当作函数参数来调用，并返回一个值, 比如 `thread::spawn()`


## 为它实现某个`trait`
这样它也能够表现出其他行为，而不仅仅是作为函数被调用。  
比如有些接口既可以传入一个结构体，也可以传入一个函数或者闭包


# 闭包的类型

- `FnOnce` 只能被调用一次  
- `FnMut` 允许在执行时修改闭包的内部数据，可以执行多次
- `Fn` 不能修改闭包的内部数据，可以执行多次

个人理解，闭包无论是哪种类型，在编译后，都会被转换成一个结构体，被看作是一个新的类型，并必须遵守rust的所有权与借用规则.  
既然要遵循规则，就得分为 所有权、引用、可变引用 三种情况，而 FnOnce、FnMut、Fn 三种类型则是对这三种情况的一种封装.

- `FnOnce` 类型：参数为`self`, 因此该结构体所有权被转移，闭包只能被调用一次，不能修改闭包内部数据  
- `FnMut` 类型：参数为`&mut self`，因此该结构体可变借用，闭包可以被调用多次，可以修改闭包内部数据  
- `Fn` 类型：参数为`&self`，因此该结构体不可变借用，闭包可以被调用多次，不能修改闭包内部数据  

而闭包的类型，是由实际的被捕获的变量的使用方式决定的，而不是靠显式声明：
比如你捕获了变量，但是没有改变它的值，那么它就是 `FnOnce`或`Fn`类型；  
如果你进一步、多次调用该闭包，那么它就会被编译器推断为`Fn`类型；  
一旦你试图修改闭包内部数据，那么它就会被编译器推断为`FnMut`类型。  
而这明显不是靠显式声明的，如果显式声明不符合实际情况，编译器会报错，你还是得重新修改类型