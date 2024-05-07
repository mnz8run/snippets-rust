## Cargo example 默认情况

默认情况下，Cargo 会在 `examples` 目录中查找示例程序（文件名以 .rs 扩展名结尾的文件）。

示例程序应该能够独立运行，通常使用 fn main() 函数作为入口点。

example_name 默认取自 examples 下 .rs 文件名。

## 指定文件

Cargo.toml 可以指定出默认情况以外的 example

```
[[example]]
name = "other_example"
path = "other_examples/other_example.rs"
```

这里的限制：必须要有 name 字段，path 必须到文件名。

## 重名的情况

examples 中和 Cargo.toml 中指定的 `[[example]]` 重复，则取 Cargo.toml 中的。

例子：

```
cargo_example
├── examples
│   └── example.rs
├── other_examples
│   └── example.rs
└── Cargo.toml
```

Cargo.toml 文件：

```
[[example]]
name = "example"
path = "other_examples/example.rs"
```

## 需要参数

cargo run --example exname -- --exoption exarg1 exarg2

`--`：这个双破折号表示 Cargo 选项的结束和示例（exname example）选项的开始。
