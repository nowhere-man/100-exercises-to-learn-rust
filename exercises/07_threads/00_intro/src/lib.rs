// 学习笔记：docs/07_threads/00_intro/learning.md
// 测试命令： cargo test --manifest-path exercises/07_threads/00_intro/Cargo.toml

fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to _!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(
            intro(),
            "I'm ready to build a concurrent ticket management system!"
        );
    }
}
