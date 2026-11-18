# Visibility：模块边界与成员可见性

[对应源码](../../../exercises/03_ticket_v1/04_visibility/src/lib.rs)

## Package、crate 与模块

Cargo package 是由 `Cargo.toml` 管理的项目单元，可以包含一个库 crate 和多个二进制 crate。crate 是 Rust 的编译单元，可以由多个 `.rs` 文件组成；模块是 crate 内的代码组织层级。

这与 C++ 的编译单元边界不同：C++ 通常按 `.cpp` 加预处理展开内容形成编译单元，Rust 则把多个模块文件纳入同一个 crate。

## 默认私有与可见范围

这里的模块、结构体、字段和方法默认私有。**私有项在定义所在模块及其所有层级的子模块中可见，整个 crate 内的其他模块不一定能访问。**

“直接子模块”指下一层；“后代模块”指所有向下嵌套的模块。下面的范围都包含相应模块的所有层级子模块。

| 修饰符 | 可见范围 |
|---|---|
| 不写 | 定义所在模块及其所有层级子模块 |
| `pub(crate)` | 当前 crate |
| `pub(super)` | 父模块及其所有层级子模块 |
| `pub(in crate::ancestor)` | 指定祖先模块及其所有层级子模块 |
| `pub` | 项本身公开，但访问路径仍须可达 |

`pub(in ...)` 必须指定祖先模块，不能任意指定兄弟模块。Rust 2018 及之后的这种路径从 `crate`、`self` 或 `super` 开始。

## `super` 是定义位置的父模块

```rust
// src/lib.rs
mod model {
    pub(super) fn hello() {}
}
```

`model` 是 crate 根的直接子模块，`hello` 上的 `super` 指向 crate 根。因此这里 `pub(super)` 与 `pub(crate)` 的范围相同。若函数位于更深的模块中，`super` 只向上一级，范围就会缩小。

## 类型公开不代表字段公开

**类型、字段和方法分别声明可见性。** Rust 没有 C++ 的 `public:`、`private:` 分区；每个声明旁边的修饰符直接表明权限，公开外层类型不会意外公开内部成员。

本题中 `ticket` 与 `tests` 是兄弟模块。`Ticket` 和 `Ticket::new` 的 `pub(super)` 允许 `tests` 使用它们，但三个私有字段仍只在 `ticket` 及其所有层级子模块中可见。

所以 `tests` 可以调用构造方法，却不能读取 `ticket.description`，也不能直接写 `Ticket { ... }`。题目要求确认这些操作无法编译后注释掉它们，而不是公开字段。

## 外部访问还要有公开路径

即使 `Ticket` 写成 `pub`，若外部必须经过私有的 `ticket` 模块，原路径仍不可达。可以公开模块，也可以使用 `pub use ticket::Ticket;` 重新导出类型。
