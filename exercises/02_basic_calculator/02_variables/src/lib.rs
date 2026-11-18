// 学习笔记：docs/02_basic_calculator/02_variables/learning.md
// 测试命令： cargo test --manifest-path exercises/02_basic_calculator/02_variables/Cargo.toml

// 👇 The lines below, starting with `///`, are called **documentation comments**.
//    They attach documentation to the item that follows them. In this case, the `speed` function.
//    If you run `cargo doc --open` from this exercise's directory, Rust will generate
//    HTML documentation from these comments and open it in your browser.

/// Given the start and end points of a journey, and the time it took to complete it,
/// calculate the average speed.
/*
Rust 中的每个变量都必须具有一个类型，这个类型可以由编译器推断定，或者由开发者显式指定。
语法： let <variable_name>: <type> = <expression>;

如果我们不指定变量的类型，编译器会根据变量使用的上下文尝试推断其类型。
let x = 42;
let y = x;
let z: u16 = x; // error


函数参数也是变量
函数参数和用 let 声明的变量唯一区别在于，函数参数必须显式声明类型。编译器不会为你推断。


声明变量时不必立即初始化。但是，在使用变量之前必须初始化它。否则编译器会报错
*/
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: define a variable named `distance` with the right value to get tests to pass
    //  Do you need to annotate the type of `distance`? Why or why not?
    let distance: u32 = end - start;
    // Don't change the line below
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
