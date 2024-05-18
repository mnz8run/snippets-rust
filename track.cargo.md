[The Cargo Book](https://doc.rust-lang.org/cargo/index.html)

[Rust 学习-- Cargo 配置详解](https://sdk.nnsdao.com/docs/rust-guide/rust-cargo-config-detail/)

## Cargo 的默认情况

|                |              | 限制                     |                                |
| -------------- | ------------ | ------------------------ | ------------------------------ |
| package name   | 包名         | 不能数字开头，不能有空格 |                                |
| directory name | 包的文件夹名 |                          |                                |
| binary name    | 二进制名     |                          | 可以在 Cargo.toml 单独制定名字 |
| library name   | 库名         | 不能使用中划线 `-`       | 可以在 Cargo.toml 单独制定名字 |

- 默认情况：
  1. 包名取自包的文件夹名
  2. 二进制名取自包名
  3. 库名取自包名

库名：在默认的库名取自包名情况下，如果在包名含有中划线的情况，库名将会默认转化为下划线 `_`

包名：characters must be Unicode XID characters (numbers, `-`, `_`, or most letters)

## features (特性)

### 特性是什么？

在 Cargo 中，features 是一个可选的编译时功能，它可以添加代码、改变代码的行为或者干脆禁用某些代码。
开发者可以根据自身的需求，为不同的目的、平台或配置选择不同的 features。
实际上，我们可以把它看作是一个能够按需求启用关闭功能的开关。

```toml
[package]
name = "my_project"
version = "0.1.0"

[features]
default = ["feat_a"]
feat_a = []
feat_b = []
```

在这个例子中，定义了两个特性：feat_a 和 feat_b。default 是一个特殊的特性，它会在没有指定特性的情况下启用。在这里，我们使得 feat_a 默认启用。

### 特性如何改变代码的行为？

特性可以通过 Rust 的 cfg 属性来改变代码的行为。这是一个条件编译属性，它允许我们选择性地包含或排除部分代码。

```rs
#[cfg(feature = "feat_a")]
fn function_a() {
    println!("Function A is enabled");
}

#[cfg(feature = "feat_b")]
fn function_b() {
    println!("Function B is enabled");
}
```

在这个例子中，function_a 只有在启用 feat_a 特性时才会编译，同样，function_b 只有在启用 feat_b 特性时才会编译。

### 如何启用和禁用特性？

在编译时，我们可以使用--features 参数来启用特性。例如，我们可以使用 cargo build --features "feat_b"来启用 feat_b 特性。
如果我们想要禁用所有默认特性，我们可以使用--no-default-features 参数。

```
cargo build --features "feat_b"
cargo build --no-default-features
```

### 特性之间的依赖关系

特性之间可以有依赖关系，例如，我们可以使得 feat_b 依赖于 feat_a。

```toml
[features]
feat_a = []
feat_b = ["feat_a"]
```

在这个例子中，如果我们启用 feat_b 特性，那么 feat_a 特性也会自动启用。
