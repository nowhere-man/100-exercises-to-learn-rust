// 学习笔记：docs/05_ticket_v2/11_dependencies/learning.md
// 测试命令： cargo test --manifest-path exercises/05_ticket_v2/11_dependencies/Cargo.toml

// TODO: Add `anyhow` as a dependency of this project.
//  Don't touch this import!

// When you import a type (`Error`) from a dependency, the import path must start
// with the crate name (`anyhow`, in this case).
use anyhow::Error;
