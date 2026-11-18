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
#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}
