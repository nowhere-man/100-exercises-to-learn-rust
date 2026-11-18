// 学习笔记：docs/05_ticket_v2/00_intro/learning.md
// 测试命令： cargo test --manifest-path exercises/05_ticket_v2/00_intro/Cargo.toml

fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to __!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to refine the `Ticket` type!");
    }
}
