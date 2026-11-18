// 学习笔记：docs/02_basic_calculator/03_if_else/learning.md
// 测试命令： cargo test --manifest-path exercises/02_basic_calculator/03_if_else/Cargo.toml

/// Return `12` if `n` is even,
/// `13` if `n` is divisible by `3`,
/// `17` otherwise.
/*
如果 if 表达式中的条件不是布尔值，你会遇到编译错误。
这源自 Rust 的类型强制哲学：不存在从非布尔类型到布尔类型的自动转换。  Rust 不具备 真值 或 假值 的概念。 你需要明确你想检查的条件。

没有 else 的 if 要求分支产生 ()。
() 是 Rust 的“单元类型”，表示没有有意义的结果。如果条件为假，这个 if 就产生 ()；条件为真时也必须匹配这个类型。

一个 if 的不同执行路径，必须产生相同类型的值。
()：它叫单元类型（unit type），这个类型只有一个值，也写作 ()。你可以把它理解为“没有有意义的结果”。它的用途类似 C/C++ 的 void，但 Rust 的 () 是一个真正的类型，也有对应的值。
if n % 2 == 0 {
    12
}
等价于
if n % 2 == 0 {
    12
} else {
    () // 没有 else 时的结果
}
一条路径产生整数，另一条产生 ()，类型不一致，所以编译器会拒绝。

但是
if n % 2 == 0 {
    return 12;
}
和
if n % 2 == 0 {
    12;
}
也不一样，

前者：计算出 12，然后丢弃它。分支正常执行到末尾，没有留下值，所以结果是 ()
后者：return 会直接退出整个函数，这个分支根本不会正常执行到末尾，因此不是“算出 12，丢弃它，再得到 ()”。
Rust 用 `!`（never type，永不类型）表示这种不会正常完成的表达式。`return 12` 的类型是 !，它可以在要求 () 的位置使用，因为这条路径不会继续执行，也就不会产生一个类型不匹配的结果。
*/
fn magic_number(n: u32) -> u32 {
    if n % 2 == 0 {
        return 12;
    }
    if n % 3 == 0 {
        13
    } else {
        17
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
