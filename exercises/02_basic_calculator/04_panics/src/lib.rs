// 学习笔记：docs/02_basic_calculator/04_panics/learning.md
// 测试命令： cargo test --manifest-path exercises/02_basic_calculator/04_panics/Cargo.toml

/// Given the start and end points of a journey, and the time it took to complete the journey,
/// calculate the average speed of the journey.
/*
panic是Rust表明出现了严重错误，程序无法继续执行，是不可恢复的错误
通过调用 panic! 宏故意触发恐慌: panic!("This is a panic!");

#[should_panic(expected = "The journey took no time at all. That's impossible!")]
测试用来预期执行的代码会产生panic错误
*/
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: Panic with a custom message if `time_elapsed` is 0
    if time_elapsed == 0 {
        panic!("The journey took no time at all. That's impossible!");
    }
    return (end - start) / time_elapsed;
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 With the `#[should_panic]` annotation we can assert that we expect the code
    //    under test to panic. We can also check the panic message by using `expected`.
    //    This is all part of Rust's built-in test framework!
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
