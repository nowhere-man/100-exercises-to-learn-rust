# Overflow：整数环绕与 Cargo 编译配置

[对应源码](../../../exercises/02_basic_calculator/08_overflow/src/lib.rs)

## 超出整数范围与环绕

结果大于类型最大值称为上溢，小于类型最小值称为下溢；这里统称整数溢出。

**环绕会保留固定宽度内的位，跨过边界后从另一端继续。** 例如 `u8` 中 `255 + 1` 环绕为 `0`，`0 - 1` 环绕为 `255`。

本题的 `20!` 超出 `u32` 范围，练习要求乘法环绕后得到 `2_192_834_560`，并非精确的阶乘值。

## `overflow-checks` 控制普通整数运算

对于本题的运行时整数乘法，`overflow-checks = true` 会在溢出时触发 `panic`，`false` 则让结果环绕。它不是用来选择饱和算术的开关，也不意味着关闭后所有编译期溢出都被允许。

通常应优先发现意外溢出，避免悄悄产生错误结果；本题关闭检查是为了学习环绕行为，不是普遍推荐。

## Profile 是编译配置集合

Cargo 的 profile 是一组编译选项，**不是分别创建 `dev.toml`、`release.toml` 文件**。在 `Cargo.toml` 中写配置段即可覆盖默认选项。

| 命令 | 使用的 profile | 默认溢出检查 |
|---|---|---|
| `cargo build`、`cargo run` | `dev`，侧重编译速度与调试体验 | 开启 |
| `cargo build --release`、`cargo run --release` | `release`，侧重运行性能 | 关闭 |
| `cargo test` | `test`，默认继承 `dev` | 继承 `dev` |
| `cargo test --release` | `release` | 关闭 |

例如，分别自定义开发和发布模式：

```toml
[profile.dev]
overflow-checks = false

[profile.release]
overflow-checks = true
```

## 工作空间根配置与包级覆盖

**在这个工作空间中，profile 必须写在仓库根目录的 `Cargo.toml`；练习包自己的设置会被忽略。** `--manifest-path` 指定练习也不改变这条规则。

只针对本题关闭检查，可以使用根目录中已有的配置：

```toml
[profile.dev.package.overflow]
overflow-checks = false
```

`package.overflow` 匹配包清单中的 `name = "overflow"`，会自动覆盖这个包的通用 `dev` 设置。`package.copy` 则只匹配名为 `copy` 的包。**包级覆盖不是新建 profile，不需要单独选择它。**
