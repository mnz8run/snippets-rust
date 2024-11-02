## run

### cargo run 在 workspace 根目录运行的默认行为

1. **单一可执行 crate**：如果 workspace 中只有一个 crate 是可执行的（带有 `main.rs` 文件），则直接运行 `cargo run` 会默认执行这个 crate。

2. **多个可执行 crate**：如果 workspace 中有多个可执行的 crate，会出现 error 提示。

如果希望 `cargo run` 默认运行其中一个，可以在 workspace 的 `Cargo.toml` 中使用 `[workspace.metadata]` 来指定。例如：

```toml
[workspace]
members = ["crate1", "crate2"]

[workspace.metadata.default-run]
default-run = "crate1"
```

这样在根目录运行 `cargo run` 时，它会默认运行 `crate1`。

### package

运行指定包。

cargo run -p [package_name]

### binary

运行指定二进制文件。

cargo run --bin [binary-name]

### example

运行指定示例，在 workspace 下的所有 example 都可以指定。

cargo run --example [example-name]

## 创建 workspace

创建文件 Cargo.toml 。

```toml
[workspace]
members = []
```

members 字段填写包的文件夹路径。

> `The members list also supports globs to match multiple paths, using typical filename glob patterns like * and ?.`

[Pattern](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html)

## 创建 member

`cargo new packages/sample --name sample --vcs none`

由于限制（看 [track cargo](./track.cargo.md) ），所以：

- 包的文件夹名统一为：不以数字开头，使用下划线。
- 包名：不以数字开头，使用下划线。
- 二进制名：不以数字开头，使用下划线。
- 库名：不以数字开头，使用下划线。

记得原先需要手动加入 members 中，现在可以自动加入 members 字段中。new 和 add 都可以。

```
rustup 1.27.0 (bbb9276d2 2024-03-08)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.77.2 (25ef9e3d8 2024-04-09)`
rustc 1.77.2 (25ef9e3d8 2024-04-09)
cargo 1.77.2 (e52e36006 2024-03-26)
```

如果在已有配置中，也不会再新增。

```
[workspace]
members = ["packages/*"]
```

例如，上面的配置，在 packages 新增，不会新增冗余。

### cargo init vs cargo new

区别在于，如果是已存在的文件夹，且没有 Cargo.toml 文件时，cargo-init 可以执行，cargo-new 不能执行。

```
cargo init packages/sample
cargo new packages/sample
```

`packages/sample` 文件夹存在，没有 Cargo.toml 文件，cargo-init 可以执行，cargo-new 不能执行。

如果有 Cargo.toml 文件，两个命令都不能执行。
