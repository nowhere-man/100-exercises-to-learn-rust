// 学习笔记：docs/03_ticket_v1/05_encapsulation/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/05_encapsulation/Cargo.toml

/*
封装是隐藏对象内部表示的做法
如果所有字段都是公开的，就没有封装。你必须假设字段可以在任何时候被修改为由其类型允许的任何值。
为了执行更严格的规则，我们必须保持字段私有。然后，我们可以提供公共方法来与Ticket实例交互。那些公共方法将负责维护我们的不变量。
如果所有字段都是私有的，那么就不能直接使用结构体实例化语法创建实例了。


访问器方法：访问器方法是公共方法，允许你读取私有结构体字段（或字段）的值。


 */
pub mod ticket {
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

        // TODO: Add three public methods to the `Ticket` struct:
        //  - `title` that returns the `title` field.
        //  - `description` that returns the `description` field.
        //  - `status` that returns the `status` field.
        pub fn description(self) -> String{
            self.description
        }
        pub fn status(self) -> String{
            self.status
        }
        pub fn title(self) -> String{
            self.title
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ticket::Ticket;

    #[test]
    fn description() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.description(), "A description");
    }

    #[test]
    fn title() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.title(), "A title");
    }

    #[test]
    fn status() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.status(), "To-Do");
    }
}
