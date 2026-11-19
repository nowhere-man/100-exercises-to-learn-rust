// 学习笔记：docs/03_ticket_v1/10_references_in_memory/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/10_references_in_memory/Cargo.toml

/*
Rust 中的大多数引用在内存中都是以指向内存位置的指针形式表示的。可以说，这是一个指向指向堆分配数据的指针的指针。&mut String 的情况也是如此。
它们的大小与指针的大小相同，即 usize。可以使用 std::mem::size_of 来验证
assert_eq!(std::mem::size_of::<&String>(), 8);
assert_eq!(std::mem::size_of::<&mut String>(), 8);


并非所有指针都指向堆
它们只是指向一个内存位置，这个位置_可能_在堆上，但不一定是。
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
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), 8);
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), 8);
    }
}
