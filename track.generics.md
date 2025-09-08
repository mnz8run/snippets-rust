[Const Generics](https://practice.course.rs/generics-traits/const-generics.html)

[Const generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics)

## turbofish

Turbofish（`::<>`）是 Rust 中的一种语法，用于在函数调用时显式指定泛型类型参数。它的名字来源于它的形状看起来像一条鱼 🐟。

### 基本语法

```rust
function_name::<Type1, Type2>(arguments)
```

### 为什么需要 turbofish？

当 Rust 编译器无法推断泛型类型时，你需要显式指定类型。这通常发生在：

1. **返回类型模糊**
2. **多个可能的类型转换**
3. **泛型函数调用**

### 何时使用 turbofish？

- **类型注解不方便时**: 比如在链式调用中
- **需要指定中间类型时**: 在复杂的泛型操作中
- **API 设计要求时**: 某些函数设计就是要求显式类型

```rust
// 链式调用中使用 turbofish 更方便
let result = vec![1, 2, 3]
    .into_iter()
    .map(|x| x * 2)
    .collect::<Vec<i32>>()  // 这里用 turbofish 比类型注解方便
    .len();
```

### 实际例子

#### 1. add 函数

```rust
use std::ops::Add;

pub fn add<T, R>(left: T, right: T) -> R
where
    T: Into<R>,
    R: Add<Output = R>,
{
    left.into() + right.into()
}

// 使用 turbofish 指定 T 和 R 的类型
let result = add::<u8, u16>(2u8, 3u8);  // T=u8, R=u16

// 等价于类型注解的方式
let result: u16 = add(2u8, 3u8);
```

#### 2. 集合类型推断

```rust
// 编译器不知道要收集成什么类型
let numbers = (0..10).collect::<Vec<i32>>();

// 或者使用类型注解
let numbers: Vec<i32> = (0..10).collect();
```

#### 3. 字符串解析

```rust
// 编译器不知道要解析成什么类型
let num = "42".parse::<i32>().unwrap();

// 等价写法
let num: i32 = "42".parse().unwrap();
```

#### 4. 类型转换

```rust
use std::convert::TryInto;

let big_num: u64 = 1000;
let small_num = big_num.try_into::<u32>().unwrap();
```
