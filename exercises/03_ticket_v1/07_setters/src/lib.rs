// 学习笔记：docs/03_ticket_v1/07_setters/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/07_setters/Cargo.toml

/*
设置器方法允许用户更改Ticket的私有字段值，同时确保其不变性得到尊重
在Rust中有两种常见的设置器实现方式：
- 将self作为输入。
- 将&mut self作为输入。

impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // validation..
        self.title = new_title;
        self
    }
}
它获取self的所有权，更改标题，并返回修改后的Ticket实例。
由于set_title获取self的所有权（即消耗它），我们需要将结果重新赋值给一个变量。
我们利用变量覆盖来重用相同的变量名：当你使用与现有变量相同的名字声明新变量时，新变量会覆盖旧变量。

当需要一次性更改多个字段时，self-设置器工作得相当不错：你可以将多个调用串联在一起！
let ticket = ticket
    .set_title("新标题".into())
    .set_description("新描述".into())
    .set_status("进行中".into());

第二种设置器方法，使用&mut self
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // validation
        self.title = new_title;
    }
}
这次，该方法以可变引用的形式接收self作为输入，更改标题，就这样。没有返回任何东西。
所有权保留在调用者手中，所以原始的ticket变量仍然是有效的。我们不需要重新分配结果。但是，我们需要将ticket标记为可变的，因为我们正在对其采取可变引用。
&mut-设置器有一个缺点：你不能链式调用多个设置。由于它们不返回修改后的Ticket实例，你不能在第一个调用的结果上再调用另一个设置器。你必须分别调用每个设置器


Q:将self作为输入的这种setter，是不是有性能损耗啊，每次都创建新的对象。 相比&mut self
A:通常没有额外的对象创建，也不一定比 &mut self 慢。`self` setter 消耗当前 Ticket，修改字段后把同一个 Ticket 返回：
Ticket 的所有权从调用者转到方法里，再从方法返回给调用者。对这个简单结构体，编译器通常能把这段过程优化掉
*/
// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

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

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_title(&mut self, new_title: String) {
        if new_title.is_empty() {
            panic!("Title cannot be empty");
        }
        if new_title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        self.title = new_title;
    }

    pub fn set_description(&mut self, new_description: String) {
        if new_description.is_empty() {
            panic!("Description cannot be empty");
        }
        if new_description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        self.description = new_description;
    }

    pub fn set_status(&mut self, new_status: String) {
        if new_status != "To-Do" && new_status != "In Progress" && new_status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
        self.status = new_status;
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
