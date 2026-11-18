// 学习笔记：docs/03_ticket_v1/00_intro/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/00_intro/Cargo.toml

fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to __!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to start modelling a software ticket!");
    }
}
