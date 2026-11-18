// 学习笔记：docs/05_ticket_v2/00_intro/learning.md

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
