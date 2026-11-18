# Saturating：按操作选择溢出行为

[对应源码](../../../exercises/02_basic_calculator/09_saturating/src/lib.rs)

## 编译配置与单次操作

`overflow-checks` 控制其配置作用范围内的普通整数运算，还可以按包覆盖，因此不宜简单理解为“整个程序只有一个不可区分的全局开关”。

**同一个程序可以在不同操作中明确选择不同的溢出行为。** `wrapping_` 与 `saturating_` 方法的语义不依赖普通运算的溢出检查开关。

## 环绕与饱和

| 方法 | 超出范围时的行为 | `u8` 示例 |
|---|---|---|
| `wrapping_add` | 环绕到范围另一端 | `255u8.wrapping_add(1)` 为 `0` |
| `saturating_add` | 停在相应边界 | `255u8.saturating_add(1)` 为 `255` |

饱和算术在结果过大时返回类型最大值，过小时返回最小值；未超出范围时正常计算。**`overflow-checks` 无法开启饱和算术，必须在操作处明确使用 `saturating_` 方法。**

## 饱和阶乘

当前实现使用 `result = result.saturating_mul(i)` 累积乘积。对于这里的正整数乘法，一旦达到 `u32::MAX`，后续乘法仍停在这个上限。

`let mut result: u32 = 1` 明确指定接收者类型，让编译器确定调用的是 `u32` 的 `saturating_mul`。
