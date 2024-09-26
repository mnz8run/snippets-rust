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

### cargo 如何处理版本的

在 Rust 的包管理器 Cargo 中，如果你的项目中有多个依赖都需要 `serde`，但是版本不同时，Cargo 会尝试找到一个能满足所有依赖版本要求的解决方案。如果可能，它会使用版本解析算法来选择一个兼容所有要求的版本。以下是具体处理方式：

1. **版本解析**：Cargo 使用 [语义化版本控制（SemVer）](https://semver.org/) 来解析依赖。根据 SemVer，如果依赖声明兼容性（如使用 `^`、`~` 或指定范围），Cargo 将尝试解析出一个版本，它同时满足这些兼容性要求。
2. **多版本共存**：如果没有单一版本可以满足所有依赖的需求（通常是因为存在重大不兼容的版本升级），Cargo 允许项目中同时存在多个版本的 `serde`。例如，如果一个依赖需要 `serde ^1.0.1` 而另一个依赖需要 `serde ^2.0.0`，Cargo 会同时下载并编译这两个不同主版本的 `serde`。
3. **编译和链接**：Cargo 编译时会处理好所有依赖之间的版本差异，确保最终生成的可执行文件或库文件在运行时不会出现版本冲突。这包括为不同版本的相同库编译不同的代码路径。
4. **依赖覆盖和替代**：如果你需要覆盖默认的依赖版本解析，可以在 `Cargo.toml` 中使用 `[dependencies]`、`[dev-dependencies]` 或 `[build-dependencies]` 来显式指定需要的版本。还可以使用 `[replace]` 或 `[patch]` 来替代已有的依赖。

总之，Cargo 提供了强大的依赖管理和版本控制功能，可以有效解决复杂的版本依赖问题。如果你在项目中遇到具体的依赖问题，可以通过调整 `Cargo.toml` 文件中的版本指定或使用更精细的依赖管理策略来解决。
