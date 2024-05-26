## closure

lambda 表达式 匿名函数

为什么会有闭包，在什么情况下需要使用闭包

ChatGPT

闭包在 Rust 中是一种匿名函数，能够捕获其定义时所在的环境中的变量。闭包的主要用途包括：

1. **简化代码**：闭包可以简化代码，减少重复，特别是在需要传递短小函数的地方。
2. **函数式编程**：闭包是函数式编程的重要特性，使得代码更具表达力和灵活性。
3. **延迟执行**：闭包可以用于延迟计算或条件执行，特别是在需要动态决定行为时。
4. **异步编程**：在异步编程中，闭包可以捕获上下文并在未来的某个时间点执行。

### 示例

```rust
fn main() {
    let x = 10;
    let add_x = |y| y + x; // 捕获环境中的 x
    println!("{}", add_x(5)); // 输出 15
}
```

### 使用场景

1. **迭代器和集合操作**：

   ```rust
   let vec = vec![1, 2, 3, 4];
   let doubled: Vec<_> = vec.iter().map(|x| x * 2).collect();
   ```

2. **异步任务和线程**：

   ```rust
   use std::thread;
   let handle = thread::spawn(|| {
       println!("Hello from a new thread!");
   });
   handle.join().unwrap();
   ```

闭包使得代码更具表达力和灵活性，特别适合需要捕获环境变量的场景。
