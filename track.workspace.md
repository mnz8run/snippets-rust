[The Cargo Book](https://doc.rust-lang.org/cargo/index.html)

## 创建 workspace

创建文件 Cargo.toml

```toml
[workspace]
members = []
```

members 字段填写包的文件夹路径。

> `The members list also supports globs to match multiple paths, using typical filename glob patterns like * and ?.`

[Pattern](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html)

## 创建 member

`cargo new packages/sample --name sample`

由于限制（看 [track](./track.md) ），所以：

- 包的文件夹名统一为：不以数字开头，使用下划线
- 包名：不以数字开头，使用下划线
- 二进制名：不以数字开头，使用下划线
- 库名：不以数字开头，使用下划线

### cargo init vs cargo new

cargo-new - Create a new Cargo package

cargo-init - Create a new Cargo package in an existing directory
