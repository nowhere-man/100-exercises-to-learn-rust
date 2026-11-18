// 学习笔记：docs/03_ticket_v1/03_modules/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/03_modules/Cargo.toml

/*
在Rust中，模块是一种将相关代码组织在一起的方式，置于一个共同的命名空间下（即模块名）
内联模块:模块声明（mod tests）和模块内容（里面的内容{ ... }）紧挨着一起。
模块可以嵌套，形成树状结构。树的根是crate本身即包含所有其他模块的顶级模块。
对于库，根模块通常是src/lib.rs

内联模块对小段代码很有用，但随着项目成长，你会想把代码拆分成多个文件。在父模块里，你用mod关键字声明子模块的存在。
mod dog;
Rust的构建工具cargo则负责找到包含模块实现的文件。如果你的模块声明在crate的根目录（如src/lib.rs或src/main.rs），cargo期待文件命名为：
src/<module_name>.rs
src/<module>/mod.rs
如果你的模块是另一个模块的子模块，文件应命名为：
[..]/<parent_module>/<module>.rs
[..]/<module>/mod.rs
比如，如果是animals的子模块，那么src/animals/dog.rs或src/og/mod.rs。

项路径和use语句:同一模块里的项可以直接访问，不需要特别语法。直接用它们的名字就行。

但如果你想从不同模块访问实体得用指向要访问实体的路径。
路径可以用多种方式组合：
1. 从当前crate根开始，比如 crate::module_1::module_2::MyStruct
2. 从父模块开始，比如 super::my_function
3. 从当前模块开始，比如 sub_module::MyStruct

每次引用类型都写全路径可能很繁琐。为了方便，你可以引入use语句来把实体引入作用域。
use crate::module_1::module_2::MyStruct;

星号导入:也可以用一个use语句导入一个模块的所有项。use crate::module_1::module_2::*;
通常不鼓励这样做因为它可能会污染当前命名空间，使得难以理解每个名字来自哪里，并且潜在地引起名称冲突。
 */
mod helpers {
    // TODO: Make this code compile, either by adding a `use` statement or by using
    //  the appropriate path to refer to the `Ticket` struct.
    use crate::Ticket;
    fn create_todo_ticket(title: String, description: String) -> Ticket {
        Ticket::new(title, description, "To-Do".into())
    }
}

struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Ticket {
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
}
