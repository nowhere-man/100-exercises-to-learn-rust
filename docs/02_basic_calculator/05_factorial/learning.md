# Factorial：不可变绑定与 `mut`

[对应源码](../../../exercises/02_basic_calculator/05_factorial/src/lib.rs)

## 绑定默认不可变

**Rust 的变量绑定默认不可变；初始化后，不能通过这个绑定重新赋值。** 与 C/C++ 中需要用 `const` 主动限制修改不同，Rust 要求用 `mut` 主动允许修改，便于看出哪些状态会变化。

```rust
let mut result = 1;
result *= 2;
```

`let` 创建绑定，`mut` 允许后续修改绑定的值。`result *= 2` 是乘法再赋值，需要 `result` 可变。

## 可变性属于各自的绑定

当前实现的 `let mut x = n` 把整数 `n` 的值复制到新绑定 `x`。之后 `x -= 1` 改变的是 `x`，不会改变参数 `n`。同样，累积乘积的 `result` 也必须声明为 `mut`。
