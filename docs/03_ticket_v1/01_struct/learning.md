# Struct：方法、关联函数与调用方式

[对应源码](../../../exercises/03_ticket_v1/01_struct/src/lib.rs)

## 在 `impl` 中定义行为

`struct` 定义数据字段，`impl Order { ... }` 定义与 `Order` 关联的行为。字段属于实例，`impl` 不会在实例中增加额外的数据字段。

**有接收者 `self`、`&self` 或 `&mut self` 的关联函数称为方法。** 接收者代表调用方法的实例；当前 `is_available(&self)` 借用实例，读取数量而不消耗对象。

```rust
impl Order {
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}
```

## 方法与关联函数的调用

方法可以使用点号调用，也可以使用类型名加路径的形式，并显式传入接收者：

```rust
order.is_available();
Order::is_available(&order);
```

没有接收者的函数称为**关联函数**，使用 `Order::函数名(参数)` 的形式调用。它类似 C++ 的静态成员函数，但 Rust 更常用“关联函数”这个术语；不能在实例后用点号把它当作方法调用。
