// 学习笔记：docs/03_ticket_v1/11_destructor/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/11_destructor/Cargo.toml


/*
作用域
变量的作用域是指Rust代码中该变量有效或存活的区域。

变量的作用域从其声明开始。当以下之一发生时结束：
1. 变量被声明的代码块（即{}之间）结束
2. 变量的所有权转移到其他人（例如，函数或其他变量）

析构函数
当一个值的所有者超出作用域时，Rust会调用其析构函数。析构函数试图清理该值使用的资源，尤其是它分配的内存。
你可以通过将值传递给std::mem::drop来手动调用析构函数。这就是为什么Rust开发者常会说某个值已经被丢弃，以此来表达一个值已经超出作用域且其析构函数已被调用。

fn main() {
    let y = "Hello".to_string();
    let x = "World".to_string();
    let h = "!".to_string();

    // Variables are dropped in reverse order of declaration
    // 隐式的
    drop(h);
    drop(x);
    drop(y);
 }


 fn compute(t: String) {
    // Do something [...]
    drop(t); // <-- Assuming `t` wasn't dropped or moved
             //     before this point, the compiler will call
             //     `drop` here, when it goes out of scope
}

fn main() {
    let s = "Hello".to_string();
    compute(s);
}

注意区别：尽管在main中调用compute后s不再有效，但在main中并没有s的drop(s)。 当你将值的所有权转移到函数时，你也正在转移清理它的责任。
这确保了一个值的析构函数至多被调用一次，设计上防止了双重释放漏洞。


如果你尝试在值被丢弃后使用它会发生编译错误
Drop消耗掉它被调用的对象，意味着调用后该对象不再有效。因此，编译器会阻止你使用它



我们只想调用一次析构函数。你可能对同一个值有多个引用
如果我们中的一个超出作用域时就调用它们指向的值的析构函数，其他引用会怎么样？ 它们会指向一个不再有效的内存位置：一个所谓的悬挂指针，是释放后使用漏洞的近亲。Rust的所有权制度从设计上排除了这类漏洞。


注意：
let x = 42i32;
let y = &x;
drop(y);

println!("{y}"); // 可以编译
原因是 y 的类型是 &i32，共享引用实现了 Copy。调用 drop(y) 时，传给 drop 的是 y 的一份拷贝；原来的 y 没有被移走，所以后面仍能用。编译器可能会给出“对 Copy 类型调用 drop 不起作用”的警告。

let s = String::from("hello");
drop(s);
println!("{s}"); // 编译错误：s 已被移走
drop(s) 会拿走 String 的所有权，销毁它持有的堆内存，所以之后不能再使用 s。
 */


// We need some more machinery to write a proper exercise for destructors.
// We'll pick the concept up again in a later chapter after covering traits and
// interior mutability.
fn outro() -> &'static str {
    "I have a basic understanding of destructors!"
}

fn destructors() {
    use std::mem::drop;
    let s:String = String::from("hello");
    println!("{s}");
    // drop(s);
    println!("{s}");
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
