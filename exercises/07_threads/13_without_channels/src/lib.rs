// 学习笔记：docs/07_threads/13_without_channels/learning.md
// 测试命令： cargo test --manifest-path exercises/07_threads/13_without_channels/Cargo.toml

// TODO: You don't actually have to change anything in the library itself!
//  We mostly had to **remove** code (the client type, the launch function, the command enum)
//  that's no longer necessary.
//  Fix the `todo!()` in the testing code and see how the new design can be used.

pub mod data;
pub mod store;
