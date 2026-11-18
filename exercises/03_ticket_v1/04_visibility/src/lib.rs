// 学习笔记：docs/03_ticket_v1/04_visibility/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/04_visibility/Cargo.toml


/*
Rust中，默认一切都是私有的。私有实体只能在以下情况下被访问：
定义它的同一个模块内部，或其后代之一

后代：明确包含所有向下嵌套的层级
子模块：直接下一层的模块

可见性修饰符
可以使用可见性修饰符来修改实体的默认可见性。一些常见的可见性修饰符包括：
  pub： 不对实体本身施加可见范围限制，但访问路径也必须可达 // 不代表外部一定能沿原路径访问它
  pub(crate)：在当前同一个crate内部公开实体，但不允许外部访问。
  pub(super)：在父模块及其后代中可访问  // 不只是“父模块自己的代码能访问”。父模块下面的兄弟模块也在它的范围内：
  pub(in path::to::module)：在指定的祖先模块及其后代模块中可访问  // 路径必须指向当前模块的祖先模块，不能任意指定一个兄弟模块。
  不写pub:在定义所在模块及其后代中可访问

package：crate的集合。
crate ：是 Rust 的编译单元，但一个 crate 可以包含多个 .rs 文件
模块 ：是 crate 内部组织代码的层级。 用mod关键字定义。


// src/lib.rs —— crate 根模块
mod model {
    pub(super) fn hello() {}
}
model 的父模块就是 crate 根模块  这里的 super 指向 crate 根。
注意：`model` 是根模块的后代，它自己不是根模块。即使它已经是最外层的命名模块，上面仍然有一个 crate 根模块。
crate 根模块（lib.rs）
└── model
    └── hello
所以：`hello` 上的 pub(super) 在这个位置与 pub(crate) 的可见范围相同：crate 根及其所有后代

Rust 的可见性修饰符要分别写在每个类型、字段或方法上，没有 C++ 那种 public:、`private:` 分区。


 */
mod ticket {
    pub(super) struct Ticket { // 结构体的可见性与字段的可见性是分别控制的。公开 Ticket，不会自动公开它的字段。
        title: String,   // 三个字段没有可见性修饰符，仍然只在 ticket 模块及其后代中可见。
        description: String,
        status: String,
    }

    impl Ticket {
        pub(super) fn new(title: String, description: String, status: String) -> Ticket {
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
}

// TODO: **Exceptionally**, you'll be modifying both the `ticket` module and the `tests` module
//  in this exercise.
#[cfg(test)]
mod tests {
    // TODO: Add the necessary `pub` modifiers in the parent module to remove the compiler
    //  errors about the use statement below.
    use super::ticket::Ticket;

    // Be careful though! We don't want this function to compile after you have changed
    // visibility to make the use statement compile!
    // Once you have verified that it indeed doesn't compile, comment it out.
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // You should be seeing this error when trying to run this exercise:
        //
        // error[E0616]: field `description` of struct `Ticket` is private
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: Once you have verified that the below does not compile,
        //   comment the line out to move on to the next exercise!
        // assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // This should be impossible as well, with a similar error as the one encountered above.
        // (It will throw a compilation error only after you have commented the faulty line
        // in the previous test - next compilation stage!)
        //
        // This proves that `Ticket::new` is now the only way to get a `Ticket` instance.
        // It's impossible to create a ticket with an illegal title or description!
        //
        // TODO: Once you have verified that the below does not compile,
        //   comment the lines out to move on to the next exercise!
        // let ticket = Ticket {
        //     title: "A title".into(),
        //     description: "A description".into(),
        //     status: "To-Do".into(),
        // };
    }
}
