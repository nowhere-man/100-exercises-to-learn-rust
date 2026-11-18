// 学习笔记：docs/03_ticket_v1/06_ownership/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/06_ownership/Cargo.toml

/*
Rust所有权系统
旨在确保：
    数据在被读取时从不被修改
    数据在被修改时从不被读取
    数据被销毁后不再被访问
这些约束由借用检查器强制执行，它是Rust编译器的一个子系统

所有权是Rust中的一个关键概念,所有权使得Rust能够在不牺牲性能的情况下提供内存安全性。
    没有运行时垃圾收集器
    作为开发者，你很少需要直接管理内存
    你无法造成悬挂指针、重复释放以及其他与内存相关的错误

所有者:在Rust中，每个值都有一个所有者，在编译时静态确定。在任何给定时间，每个值只有一个所有者。

移动语义:所有权可以转移。你可以将其所有权转移到另一个变量
let a = String::from("hello");
let b = a; // a 不能再使用；b 接管了原字符串
Rust的所有权系统内置于类型系统中：每个函数都必须在其签名中声明它打算如何与其参数交互。
我们所有的方法和函数都消耗了它们的参数：它们获取了参数的所有权。 例如：
impl Ticket {
    pub fn description(self) -> String {  // 这里的 self 表示方法接收整个 Ticket 的所有权
        self.description
    }
}
Ticket::description 获取调用它的Ticket实例的所有权。这被称为移动语义：值(self)的所有权从调用者转移到被调用者，调用者不能再使用它了。

if ticket.status() == "待办" {
    println!("你的下一个任务是: {}", ticket.title());
}
当我们调用ticket.status()时，事件序列如下：
1. Ticket::status 获取Ticket实例的所有权
2. Ticket::status 从self提取status并将status的所有权转移回调用者
3. 剩余的Ticket实例部分被丢弃（title和description） // 按值传参就会移动，编译器随后禁止继续使用原来的 ticket 绑定。
当我们尝试再次通过ticket.title()使用ticket时，编译器会抱怨：ticket值现在已经没了，我们不再拥有它，因此不能再使用它。


借用:拥有一些不获取其所有权就能读取变量值的方法是可取的。。在Rust中，这是通过借用来完成的。
每次你借用一个值时，都会得到它的引用。引用带有它们的权限标签：
    不可变引用(&)允许你读取值，但不允许修改它
    可变引用(&mut)允许你读取并修改值

回到Rust所有权系统的目标：数据在被读取时从不被修改 & 数据在被修改时从不被读取
为了确保这两点，Rust必须对引用引入一些限制：
    你不能同时拥有对同一值的可变引用和不可变引用
    你不能同时拥有对同一值的多个可变引用
    所有者在值被借用期间不能修改值
    只要有不可变引用，你可以拥有任意数量的不可变引用，只要没有可变引用

在某种程度上，你可以将不可变引用视为值上的“只读”锁，而可变引用则像是“读写”锁。所有这些限制都由借用检查器在编译时强制执行。


通过在变量前添加&或&mut，你就是在借用它的值。
但要注意！相同的符号（&和&mut）在类型的前面有不同的含义：它们表示原始类型的引用，即引用类型本身。

借用限制通常持续到引用最后一次使用，而不是变量所在作用域结束。

赋值是复制还是移动，取决于类型是否实现了 Copy。
    对于整数、`bool` 等实现 Copy 的类型，赋值会复制值：
    对于 String、自定义struct 等没有实现 Copy 的类型，按值赋值会移动所有权：
    按值赋值时，`Copy` 类型复制，非 Copy 类型移动
 */



// TODO: based on what we just learned about ownership, it sounds like immutable references
//   are a good fit for our accessor methods.
//   Change the existing implementation of `Ticket`'s accessor methods to take a reference
//   to `self` as an argument, rather than taking ownership of it.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    // &self	    借用整个 Ticket，不拿走所有权
    //-> &String	返回字符串的引用，不拿走字符串
    // &self.title	借用 title 字段
    // 引用可以理解为“访问已有值的凭证”，不负责拥有或释放被引用的数据。
    // Ticket 仍然由调用者拥有，因此可以连续读取三个字段。编译器还会保证这个引用不会比它指向的数据活得更久，避免悬挂引用。
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        // If you change the signatures as requested, this should compile:
        // we can call these methods one after the other because they borrow `self`
        // rather than taking ownership of it.
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
