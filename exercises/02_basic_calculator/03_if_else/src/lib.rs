// 学习笔记：docs/02_basic_calculator/03_if_else/learning.md
// 测试命令： cargo test --manifest-path exercises/02_basic_calculator/03_if_else/Cargo.toml

/// Return `12` if `n` is even,
/// `13` if `n` is divisible by `3`,
/// `17` otherwise.
/*
如果 if 表达式中的条件不是布尔值，你会遇到编译错误。
这源自 Rust 的类型强制哲学：不存在从非布尔类型到布尔类型的自动转换。  Rust 不具备 真值 或 假值 的概念。 你需要明确你想检查的条件。
*/
fn magic_number(n: u32) -> u32 {
    if n % 2 == 0 {
        return 12;
    } else if n % 3 == 0 {
        return 13;
    } else {
        return 17;
    }
}

#[cfg(test)]
mod tests {
    use crate::magic_number;

    #[test]
    fn one() {
        assert_eq!(magic_number(1), 17);
    }

    #[test]
    fn two() {
        assert_eq!(magic_number(2), 12);
    }

    #[test]
    fn six() {
        assert_eq!(magic_number(6), 12);
    }

    #[test]
    fn nine() {
        assert_eq!(magic_number(9), 13);
    }

    #[test]
    fn high() {
        assert_eq!(magic_number(233), 17);
    }
}
