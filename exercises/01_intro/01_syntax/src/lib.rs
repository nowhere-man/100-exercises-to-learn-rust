// 学习笔记：docs/01_intro/01_syntax/learning.md
// 测试命令： cargo test --manifest-path exercises/01_intro/01_syntax/Cargo.toml

// TODO: fix the function signature below to make the tests pass.
//  Make sure to read the compiler error message—the Rust compiler is your pair programming
//  partner in this course and it'll often guide you in the right direction!
//
// The input parameters should have the same type of the return type.
/*
Rust支持多个位宽的整数:8、16、32、64、128。
用32位,u32可以表示从0到2^32 - 1(又称u32::MAX)的数字
(i32)可以表示从-2^31到2^31 - 1的数字(即从i32::MIN到i32::MAX)。

字面值:Rust编译器会尝试根据使用环境来推断字面值的类型。如果你不提供任何上下文信息,编译器将默认为i32整数字面值。
      如果你想使用其他类型,你可以在字面值后面添加所需的整数类型作为后缀——例如2u64表示一个显式声明为u64类型的2。
      可以使用下划线_来提高大数字的可读性。例如,1_000_000和1000000是相同的。

没有自动类型强制转换
  Rust对类型强制转换非常严格。即使转换是无损的,它也不会自动将一个值从一种类型转换为另一种类型3,你必须显式地进行转换。
*/
fn compute(a: u32, b: u32) -> u32 {
    // Don't touch the function body.
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}