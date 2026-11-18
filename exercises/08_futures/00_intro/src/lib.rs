// 学习笔记：docs/08_futures/00_intro/learning.md

fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to _!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to learn about futures!");
    }
}
