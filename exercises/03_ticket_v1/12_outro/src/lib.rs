// 学习笔记：docs/03_ticket_v1/12_outro/learning.md
// 测试命令： cargo test --manifest-path exercises/03_ticket_v1/12_outro/Cargo.toml

// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    pub fn new(product_name: String, quantity: i32, unit_price:i32) -> Self {
        if product_name.is_empty() || product_name.len() > 300 {
            panic!("product name error");
        }
        if quantity <= 0 {
            panic!("quantity error");
        }
        if unit_price <= 0 {
            panic!("unit_price error");
        }

        Order {
            product_name,
            quantity,
            unit_price
        }
    }

    pub fn total(&self) -> i64{
        let t:i64 =  (self.quantity as i64) * (self.unit_price as i64);
        t
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String)  {
        if product_name.is_empty() || product_name.len() > 300 {
            panic!("product name error");
        }
        self.product_name = product_name
    }

    pub fn set_quantity(&mut self, quantity: i32) {
        if quantity <= 0 {
            panic!("quantity error");
        }
        self.quantity = quantity
    }

    pub fn set_unit_price(&mut self, unit_price:i32) {
        if unit_price <= 0 {
            panic!("unit_price error");
        }
        self.unit_price = unit_price
    }
}


#[cfg(test)]
mod test {
    use crate::Order;

    #[test]
    #[should_panic(expected="product name error")]
    pub(super) fn product_name_is_not_empty() {
        Order::new(String::from(""), 1, 2);
    }
}