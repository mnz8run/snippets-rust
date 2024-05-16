[Rust 学习 -- 代码测试](https://sdk.nnsdao.com/docs/rust-guide/rust-unit-test)

[通过例子学 Rust 中文版 -- 集成测试](https://rustwiki.org/zh-CN/rust-by-example/testing/integration_testing.html)

[cargo-test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

[Documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)

[Tests](https://doc.rust-lang.org/cargo/guide/tests.html)

## 单元测试

只运行单元测，可以指定 [target](https://doc.rust-lang.org/cargo/commands/cargo-test.html#target-selection) 排除 test 文件夹内的集成测试。

--lib --bins --benches --examples 等。

cargo test -p cargo_test --bins 运行 bins 下的单元测试。

## 集成测试

```bash
# 测试指定的集成测试
cargo test -p cargo_test --test shared_test -- --nocapture
# 可以指定多个
cargo test -p cargo_test --test shared_test --test integration_test -- --nocapture
```

这里的 shared_test 是文件名

```bash
# 运行所有集成测试
cargo test -p cargo_test --tests
```

## test 命令

cargo test -p cargo_test

不指定 package，运行空间内所有的测试

```bash
# 排除多个
cargo test --workspace --exclude crate_a --exclude crate_b
# 显示函数输出
cargo test -p cargo_test -- --nocapture
# 不并行运行测试，或者控制线程数量
cargo test -- --test-threads=1
# 忽略特定测试
cargo test -- --ignored
```

### 运行指定测试函数

```bash
# 运行 integration_main 测试函数
cargo test -p cargo_test integration_main
```

## 默认情况

1. 默认使用线程来并行运行所有的测试。
2. 截获测试运行过程中产生的输出，阻止他们被显示出来。
3. cargo test 运行所有的测试，包括单元测试、集成测试和文档测试。

默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。如果希望看到通过的测试中打印的值，截获输出的行为可以通过 --nocapture 参数来禁用

## 需要参数

cargo test 后面先列出的是传递给 cargo test 的参数，接着是分隔符 --，再之后是传递给测试二进制文件的参数。例如运行 cargo test --help 会提示 cargo test 的有关参数，而运行 cargo test -- --help 可以提示在分隔符 -- 之后使用的有关参数。

## assert

assert assert_eq assert_ne

## should_panice

## `Result<T, E>`

## ignore

忽略特定测试

```rs
#[test]
fn it_works() {}

#[test]
#[ignore]
fn expensive_test() {
    // 需要运行一个小时的代码
}
```
