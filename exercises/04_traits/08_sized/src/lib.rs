// 学习笔记：docs/04_traits/08_sized/learning.md
// 测试命令： cargo test --manifest-path exercises/04_traits/08_sized/Cargo.toml

pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    std::mem::size_of::<str>();
}
