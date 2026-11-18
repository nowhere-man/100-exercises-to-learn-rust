// 学习笔记：docs/02_basic_calculator/08_overflow/learning.md
// 测试命令： cargo test --manifest-path exercises/02_basic_calculator/08_overflow/Cargo.toml

// Customize the `dev` profile to wrap around on overflow.
// Check Cargo's documentation to find out the right syntax:
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// For reasons that we'll explain later, the customization needs to be done in the `Cargo.toml`
// at the root of the repository, not in the `Cargo.toml` of the exercise.

/*
如果结果小于给定整数类型的最小值，我们称其为整数下溢。
如果结果大于给定整数类型的最大值，我们称其为整数上溢。


环绕就意味着当你达到最大值时，你又从最小值开始。

overflow-checks：
Rust让你，作为开发者，选择当整数溢出时采取哪种方法。这种行为由overflow-checks配置设置控制。
如果overflow-checks设置为true，Rust在整数操作溢出时将在运行时恐慌。如果overflow-checks设置为false，Rust在整数操作溢出时将环绕。

配置文件：一个配置文件是一组配置选项，可以用来定制Rust代码的编译方式。
Cargo提供了两个内置的配置文件：dev和release
dev配置文件在每次你运行cargo build、cargo run或cargo test时使用。它旨在本地开发，因此牺牲了运行时性能以换取更快的编译时间和更好的调试体验。
release配置文件针对运行时性能进行了优化，但会导致更长的编译时间。你需要通过--release标志明确请求——例如，cargo build --release或cargo run --release。
在同一个 Cargo.toml 中写 [profile.dev]、`[profile.release]`，就能修改两种模式的设置。
[profile.dev]
overflow-checks = false

[profile.release]
overflow-checks = true
profile 设置只能写在工作空间根目录的 Cargo.toml 中；写在package 自己的 Cargo.toml 里会被忽略。


默认情况下，overflow-checks设置为：
对于dev配置文件设为true
对于release配置文件设为false

我们的建议是对两个配置文件都启用overflow-checks：宁可崩溃也不要默默地产生错误结果。
*/
pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! is 2432902008176640000, which is too large to fit in a u32
        // With the default dev profile, this will panic when you run `cargo test`
        // We want it to wrap around instead
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // A large number literal using underscores to improve readability!
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
