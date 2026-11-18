# Modules：模块树、文件布局与路径

[对应源码](../../../exercises/03_ticket_v1/03_modules/src/lib.rs)

## 模块组织命名空间

模块把相关代码放在同一个命名空间下。`mod helpers { ... }` 是内联模块，声明与内容写在一起。模块可以嵌套，形成一棵树。

crate 是 Rust 的编译单元，模块树以 crate 根模块为起点。库的根通常是 `src/lib.rs`，二进制程序的根通常是 `src/main.rs`。

## 模块可以放在独立文件中

父模块中的 `mod dog;` 声明一个由其他文件提供内容的模块。Cargo 调用编译器构建 crate，**具体模块文件由 Rust 编译器按模块布局规则查找**，不是按文件名自动把所有 `.rs` 都纳入程序。

| 声明位置 | 模块声明 | 常规文件位置，二选一 |
|---|---|---|
| `src/lib.rs` | `mod dog;` | `src/dog.rs` 或 `src/dog/mod.rs` |
| `src/animals.rs` 或 `src/animals/mod.rs` | `mod dog;` | `src/animals/dog.rs` 或 `src/animals/dog/mod.rs` |

因此，`animals` 下的 `dog` 模块不能写成注释中的 `src/og/mod.rs`；第二种正确位置仍然保留完整父模块路径。

## 路径确定要访问的项

项是模块中的函数、结构体等声明。同一模块中的项通常可以直接用名字；跨模块时需要路径，并满足可见性规则。

| 路径 | 起点 |
|---|---|
| `crate::module::Type` | 当前 crate 根 |
| `super::function` | 当前模块的父模块 |
| `self::child::Type` | 当前模块 |
| `child::Type` | 从当前作用域可解析的名称开始 |

模块嵌套不会把父模块的所有名称自动引入子模块作用域。本题的 `Ticket` 位于 crate 根，在 `helpers` 中可以写 `crate::Ticket`。

## `use` 为路径引入名称

`use crate::Ticket;` 把 `Ticket` 这个名称引入当前作用域，之后可以直接写 `Ticket::new(...)`。**`use` 不会复制定义，也不会绕过私有访问限制。**

`use crate::module::*;` 是通配符导入，会引入该模块中可访问的项。普通代码优先显式列出名称，避免来源不清和名称冲突；测试中的 `use super::*;` 则是常见的便利写法。
