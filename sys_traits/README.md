Clone / Copy trait，约定了数据被深拷贝和浅拷贝的行为；
Read / Write trait，约定了对 I/O 读写的行为；
Iterator，约定了迭代器的行为；
Debug，约定了数据如何被以 debug 的方式显示出来的行为；
Default，约定数据类型的缺省值如何产生的行为；
From<T> / TryFrom<T>，约定了数据间如何转换的行为。