// 学习笔记：docs/02_basic_calculator/05_factorial/learning.md
// 测试命令： cargo test --manifest-path exercises/02_basic_calculator/05_factorial/Cargo.toml

// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
//
// Use only what you learned! No loops yet, so you'll have to use recursion!

/*
Rust中的变量默认是不可变的。

一旦赋值后，就不能改变其值。

如果你想允许修改，就需要使用mut关键字声明变量为可变：

*/
fn factorial(n: u32) -> u32 {
    let mut x = n;
    let mut result = 1;
    while x > 0 {
        result *= x;
        x -= 1;
    }
    return result;
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
