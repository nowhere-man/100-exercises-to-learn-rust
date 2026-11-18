// 学习笔记：docs/03_ticket_v1/00_intro/learning.md

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
