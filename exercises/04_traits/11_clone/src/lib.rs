// 学习笔记：docs/04_traits/11_clone/learning.md
// 测试命令： cargo test --manifest-path exercises/04_traits/11_clone/Cargo.toml

// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket, ticket.summary())
}

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
