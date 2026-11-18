// 学习笔记：docs/02_basic_calculator/09_saturating/learning.md
// 测试命令： cargo test --manifest-path exercises/02_basic_calculator/09_saturating/Cargo.toml

/*

overflow-checks是一个全局设置，影响整个程序。
通常情况下，你可能希望根据不同的上下文来区别处理整数溢出：有时环绕是正确的选择，而有时恐慌更为可取。

可以通过使用 wrapping_ 方法在每次操作的基础上选择执行环绕算术： let sum = x.wrapping_add(y);
可以使用 saturating_ 方法选择执行饱和算术：let sum = x.saturating_add(y); 饱和算术不会进行环绕，而是会返回整数类型的最大或最小值。

你无法通过 overflow-checks 配置文件设置来获得饱和算术——在执行算术操作时，你必须明确选择它。
 */
pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        // Use saturating multiplication to stop at the maximum value of u32
        // rather than overflowing and wrapping around
        result = result.saturating_mul(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
    }

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
