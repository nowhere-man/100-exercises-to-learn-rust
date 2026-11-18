# Panic：终止执行与预期恐慌测试

[对应源码](../../../exercises/02_basic_calculator/04_panics/src/lib.rs)

## 用 `panic!` 表达无法继续的错误

`panic`（恐慌）表示当前执行遇到了无法按正常路径处理的错误。调用 `panic!("错误说明")` 可以主动触发它。

本题在 `time_elapsed == 0` 时主动触发 `panic!`，避免继续做除零运算，并给出明确的错误信息。

**触发 `panic` 不等于在所有场景下立即终止整个进程。** 默认通常会展开调用栈，结束当前线程；若配置为中止策略，则直接终止进程。这里应记住的是：触发点不会正常继续执行，也不会正常返回速度值。

## 测试预期的 `panic`

`#[should_panic]` 标在测试函数上，表示测试代码应触发 `panic`；没有触发反而会失败。

```rust
#[test]
#[should_panic(expected = "The journey took no time at all. That's impossible!")]
fn by_zero() {
    speed(0, 10, 0);
}
```

`expected` 进一步检查恐慌消息是否**包含**指定文本，不要求整条消息完全相等。这能确认触发的是预期错误，而不是其他意外错误。
