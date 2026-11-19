# Setters：通过所有权或可变借用修改值

[对应源码](../../../exercises/03_ticket_v1/07_setters/src/lib.rs)

## 两种 setter 接口

setter 用来修改私有字段，同时在修改前验证新值，维护类型的不变条件。Rust 常见两种写法：

```rust
fn set_title(mut self, title: String) -> Self { /* 修改并返回 self */ }
fn set_title(&mut self, title: String) { /* 通过可变借用修改 */ }
```

- `self -> Self` 获取并返回值的所有权。调用者要接住返回值，适合链式调用；它不意味着每次都创建一个新对象。
- `&mut self` 借用并修改原值，所有权留在调用者手中，但调用时绑定必须可变，通常逐个调用 setter。

两者主要表达不同的所有权和调用方式。简单的 `self` setter 通常不会额外分配一个新的结构体；字段赋值本身可能替换并释放旧字段值，这两种写法都一样。

## 保持验证规则一致

构造函数和 setter 都必须拒绝不合法字段值，否则对象创建时合法、修改后却可能违反不变条件。本练习的标题、描述和状态校验在 `new` 与对应 setter 中重复；若后续规则变多，可以提取私有校验函数复用。
