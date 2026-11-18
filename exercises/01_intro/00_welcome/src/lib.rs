// 学习笔记：docs/01_intro/00_welcome/learning.md

// This is a Rust file. It is a plain text file with a `.rs` extension.
//
// Like most modern programming languages, Rust supports comments. You're looking at one right now!
// Comments are ignored by the compiler; you can leverage them to annotate code with notes and
// explanations.
// There are various ways to write comments in Rust, each with its own purpose.
// For now we'll stick to the most common one: the line comment.
// Everything from `//` to the end of the line is considered a comment.

// Exercises will include `TODO`, `todo!()` or `__` markers to draw your attention to the lines
// where you need to write code.
// You'll need to replace these markers with your own code to complete the exercise.
// Sometimes it'll be enough to write a single line of code, other times you'll have to write
// longer sections.
//
// If you get stuck for more than 10 minutes on an exercise, grab a trainer! We're here to help!
// You can also find solutions to all exercises in the `solutions` git branch.
/*

&：借用，表示返回一个引用，还表示这是一个共享的、不可变的引用，所以不能通过它修改被引用的数据。
'static：生命周期名称，表示这个引用至少在整个程序运行期间都有效

&'static str 的意思是:指向一段生命周期为 'static 的字符串切片的引用
'static 表示的是生命周期最长的一类引用，并不等同于“不可变”。真正的不可变性来自 &；如果是可变引用，会写成： &'static mut str
& 不会让数据本身变成永久不可变，它只表示“不能通过这个引用修改数据”
*/
fn greeting() -> &'static str {
    // TODO: fix me 👇
    // 注意这里没有分号，如果加上分号：实际上会返回空元组 ()，编译会报错
    // 在 Rust 中，"hello"是一个，表达式有值，，它的值是字符串切片
    // 加上分号后： "hello"; 它就被当成了一条表达式语句。这条语句会计算 "hello"，但计算结果被丢弃，整条语句的结果是 ()
    "I'm ready to learn Rust!"
    // 隐式返回:Rust 函数会把最后一个表达式的值作为返回
}

// Your solutions will be automatically verified by a set of tests.
// You can run these tests directly by invoking the `cargo test` command in your terminal,
// from the root of this exercise's directory. That's what the `wr` command does for you
// under the hood.
//
// Rust lets you write tests alongside your code.
// The `#[cfg(test)]` attribute tells the compiler to only compile the code below when
// running tests (i.e. when you run `cargo test`).
// You'll learn more about attributes and testing later in the course.
// For now, just know that you need to look for the `#[cfg(test)]` attribute to find the tests
// that will be verifying the correctness of your solutions!
//
// ⚠️ **DO NOT MODIFY THE TESTS** ⚠️
// They are there to help you validate your solutions. You should only change the code that's being
// tested, not the tests themselves.

/*
- #[...] 是 Rust 的属性（attribute）语法，用于给编译器提供额外指令。
- cfg 表示条件编译
- #[cfg(test)] 表示：下面这个模块只在编译测试时参与编译，普通 cargo build 时不包含它.
- mod tests 定义一个名叫 tests 的模块。模块用于组织代码和控制可见性  tests 是约定俗成的名称
*/
#[cfg(test)] // 编译器提供的条件编译属性
mod tests {
    /*
    - use：把某个路径下的名字引入当前作用域。
    - crate：当前编译单元，通常称为“箱”。
    - crate::greeting：当前 crate 根模块中的 greeting 函数。

    Rust 按模块划分作用域，同一个文件里也可以有不同的模块。
       外层：crate 根模块： fn greeting()
       内层：tests 子模块，有自己的作用域：mod tests
    父模块中的函数名不会自动引入子模块的作用域，所以需要说明要调用哪个 greeting。
    三种等价写法：
       方式一：从 crate 根模块引入
           use crate::greeting;
           之后调用 greeting()
       方式二：从父模块引入
           use super::greeting;
           之后调用 greeting()
       方式三：不引入，调用时写路径
           assert_eq!(super::greeting(), "I'm ready to learn Rust!");

    作用域和访问权限是两回事：Rust 允许子模块访问父模块的私有函数，因此这里的 greeting 不需要加 pub；但调用时仍需要正确的名字或路径。
    */
    use crate::greeting; // 把 greeting 引入当前模块，之后可以直接写 greeting()。

    #[test] // #[test] 编译器提供的测试标记,将这个函数标记为测试用例。Rust 的测试运行器会发现并调用
    fn test_welcome() {
        // assert_eq! 是断言宏，用来检查两个表达式的值是否相等。末尾的 ! 表示这是宏调用，与普通函数调用不同。
        // Rust 标准库提供的宏，默认就能使用
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}
