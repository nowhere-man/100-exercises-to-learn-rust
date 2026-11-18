// 学习笔记：docs/04_traits/09_from/learning.md
// 测试命令： cargo test --manifest-path exercises/04_traits/09_from/Cargo.toml

// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
