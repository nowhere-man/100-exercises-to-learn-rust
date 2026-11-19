// 学习笔记：docs/03_ticket_v1/08_stack/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/08_stack/Cargo.toml

/*
Rust中的内存布局

栈是一种后进先出（Last In, First Out, LIFO）的数据结构。当你调用一个函数时，一个新的栈帧会被添加到栈顶。这个栈帧存储了函数的参数、局部变量和一些“簿记”值。当函数返回时，这个栈帧就会从栈中弹出
从操作角度看，栈的分配/释放非常快。我们总是在栈顶推入和弹出数据，因此不需要搜索空闲内存。我们也不必担心碎片化问题：栈是一块连续的内存

Rust中的栈
Rust经常在栈上分配数据。你的函数有一个u32类型的输入参数吗？那32位就会在栈上。这一切运作得很顺利，因为这些整数的大小在编译时就已经知道了，因此编译后的程序知道需要为它们在栈上预留多少空间。
可以使用std::mem::size_of函数验证类型在栈上会占用多少字节。
assert_eq!(std::mem::size_of::<u8>(), 1);

*/
// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        assert_eq!(size_of::<u16>(), todo!());
    }

    #[test]
    fn i32_size() {
        assert_eq!(size_of::<i32>(), todo!());
    }

    #[test]
    fn bool_size() {
        assert_eq!(size_of::<bool>(), todo!());
    }
}
