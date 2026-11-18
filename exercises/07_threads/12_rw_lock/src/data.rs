// 学习笔记：docs/07_threads/12_rw_lock/learning.md
// 测试命令： cargo test --manifest-path exercises/07_threads/12_rw_lock/Cargo.toml

use crate::store::TicketId;
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
