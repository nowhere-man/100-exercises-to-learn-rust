// 学习笔记：docs/07_threads/14_sync/learning.md
// 测试命令： cargo test --manifest-path exercises/07_threads/14_sync/Cargo.toml

// Not much to be exercised on `Sync`, just a thing to remember.
fn outro() -> &'static str {
    "I have a good understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}
