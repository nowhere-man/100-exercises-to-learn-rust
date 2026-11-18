// 学习笔记：docs/02_basic_calculator/10_as_casting/learning.md
// 测试命令： cargo test --manifest-path exercises/02_basic_calculator/10_as_casting/Cargo.toml

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.

/*
强调: Rust 不会为整数执行隐式类型转换。

可以使用 as 关键字在整数类型之间进行转换:as 转换是不会失败的。
let a: u32 = 10;
let b = a as u64;
如果编译器能正确推断出来的话,你可以使用 `_` 作为目标类型
let c: u64 = a as _; // 这种转换的语义是你所期望的：所有 u32 的值都是有效的 u64 值。

如果我们反向进行
let a: u16 = 255 + 1;
let b = a as u8;
此程序将无问题运行，因为 as 转换是绝对不会失败的。
从较大的整数类型转换到较小的类型时，Rust 编译器会执行截断。
转换为 u8 时，Rust 编译器将保留 u16 内存表示的低8位  因此，256 as u8 等于 0

这在大多数场景下...不是我们期望的。 实际上，如果编译器看到你试图进行会导致截断的字面值转换，它会主动阻止你，会报错。错误：字面值超出 `i8` 的范围


使用 as 转换要非常小心：仅限于从较小类型转换到较大类型时使用。
在处理复合类型时，你将需要依赖于不同的转换机制（可失败的 和 不可失败的）
 */


#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // The compiler is smart enough to know that the value 255 cannot fit
        // inside an i8, so it'll emit a hard error. We intentionally disable
        // this guardrail to make this (bad) conversion possible.
        // The compiler is only able to pick on this because the value is a
        // literal. If we were to use a variable, the compiler wouldn't be able to
        // catch this at compile time.
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // You could solve this by using exactly the same expression as above,
        // but that would defeat the purpose of the exercise. Instead, use a genuine
        // `i8` value that is equivalent to `255` when converted to `u8`.
        let y: i8 = -1;

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1;
        assert_eq!(true as u8, v);
    }
}
