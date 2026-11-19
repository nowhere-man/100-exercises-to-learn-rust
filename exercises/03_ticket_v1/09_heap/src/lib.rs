// 学习笔记：docs/03_ticket_v1/09_heap/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/09_heap/Cargo.toml

/*
栈很棒，但它不能解决所有问题。
栈的空间往往有限（MB级别）
并且那些在编译时大小未知的数据怎么办呢？集合、字符串和其他动态大小的数据无法完全在栈上分配。这时就需要引入堆了。

堆分配
你可以将堆想象成一大块内存——如果愿意的话，就像一个巨大的数组。每当需要在堆上存储数据时，你就要向一个特殊的程序请求，即分配器，为你保留堆中的一部分。我们将这种交互（以及你保留的内存）称为堆分配。如果分配成功，分配器会给你指向已预留块起始位置的指针。

堆的结构与栈大不相同。堆分配不是连续的，它们可以位于堆内的任意位置。
跟踪堆的哪些部分正在使用，哪些部分是空闲的是分配器的工作。
然而，分配器不会自动释放你分配的内存：你需要主动去做这件事，再次调用分配器来释放不再需要的内存。

堆的灵活性是有代价的：堆分配比栈分配慢。涉及更多的管理操作！如果你阅读关于性能优化的文章，往往会建议你尽量减少堆分配，并尽可能优先使用栈上分配的数据。

String的内存布局
当你创建一个类型为String的局部变量时，Rust被迫在堆上分配：它事先不知道你要放入多少文本，因此无法在栈上预留正确大小的空间。但String并非完全堆分配，它也在栈上保留了一些数据。具体来说：
指向你在堆上预留区域的指针。 ptr
字符串的长度，即字符串中有多少字节。 len
字符串的容量，即在堆上预留了多少字节。 cap


usize
我们在栈上存储指针、长度和容量需要多少空间？这取决于你运行机器的架构。
机器上的每个内存位置都有一个地址，通常表示为无符号整数。根据地址空间的最大大小（即你的机器可以寻址多少内存），这个整数可以有不同的大小。大多数现代机器使用32位或64位地址空间。
Rust通过提供usize类型抽象了这些与架构相关的细节：一个无符号整数，其大小与在你的机器上所需寻址内存的字节数相同。在32位机器上，usize等同于u32。在64位机器上，它匹配u64。
容量、长度和指针在Rust中都表示为usize。

堆上没有std::mem::size_of
std::mem::size_of返回类型在栈上会占用的空间量，这也被称为类型的大小。
那么String在堆上管理的内存缓冲区呢？ 堆分配是String正在管理的一个资源。编译器并不将其视为String类型的一部分。

没有std::mem::size_of的等价物来衡量某个值在运行时分配的堆内存量。某些类型可能提供了检查其堆使用情况的方法（例如String的capacity方法），但在Rust中没有通用的“API”来检索运行时堆使用情况。然而，你可以使用内存分析工具来检查程序的堆使用情况

*/
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24); // len和cap是usize类型，data是*mut u8，指针类型，指针本身占8字节，指向u8字节缓冲区（char数组）
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
