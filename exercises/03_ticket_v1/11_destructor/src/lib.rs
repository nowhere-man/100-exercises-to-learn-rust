// 学习笔记：docs/03_ticket_v1/11_destructor/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/11_destructor/Cargo.toml

// We need some more machinery to write a proper exercise for destructors.
// We'll pick the concept up again in a later chapter after covering traits and
// interior mutability.
fn outro() -> &'static str {
    "I have a basic understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
