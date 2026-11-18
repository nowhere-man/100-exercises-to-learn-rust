// 学习笔记：docs/02_basic_calculator/07_for/learning.md
// 测试命令： cargo test --manifest-path exercises/02_basic_calculator/07_for/Cargo.toml

/*
for循环是一种针对迭代器中每个元素执行代码块的方式
for <element> in <iterator> {
    // code to execute
}
Rust的标准库提供了区间类型，可用于遍历一系列数字
for i in <start>..=<end> {
}
Rust中有五种类型的区间：
1: 1..5：半开区间。包含从1到4的所有数字。不包括最后一个值，即5。
2: 1..=5：包含结束值的区间。包含从1到5的所有数字，包括最后一个值，即5。
3: 1..：开放式区间。从1开始，理论上到整数最大值的所有数字（实际上是直到整数类型的上限）。
4: ..5：从整数类型的最小值开始，到4的所有数字。不包括最后一个值，即5。
5: ..=5：从整数类型的最小值开始，到5的所有数字。包括最后一个值，即5。

可以使用for循环配合前三种区间，其中起始点是明确指定的。最后两种区间类型则用于其他情境。

i 的类型由范围端点的类型决定，而端点的类型由上下文推断；没有其他约束时，整数默认是 i32
for i in 0i32..5 {
    // i 是 i32
}
for i in 0u32..5 {
    // i 是 u32
}


*/
// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
