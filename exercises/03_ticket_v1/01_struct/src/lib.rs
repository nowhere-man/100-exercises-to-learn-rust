// 学习笔记：docs/03_ticket_v1/01_struct/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/01_struct/Cargo.toml

// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.

/*
方法必须在**impl块内定义
方法可以使用self作为它们的第一个参数。 self是一个关键字，代表调用其调用方法的struct实例。

如果方法以self作为其第一个参数，它可以使用方法调用语法调用：
方法调用语法： <instance>.<method_name>(<parameters>)

如果方法不以self作为其第一个参数，它是一个静态方法。
调用静态方法的唯一方式是使用函数调用语法： <StructName>::<method_name>(<parameters>)

即使以self作为第一个参数的方法，你也可以使用函数调用语法：
<StructName>::<method_name>(<instance>, <parameters>)

 */
struct Order {
    price: u32,
    quantity: u32
}

impl Order {
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
