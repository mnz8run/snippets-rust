## projects

### packages\directory_structure

二进制目录结构示例

### packages\cargo_test

cargo test -p cargo_test

### packages\cargo_example

cargo run --example example

cargo run --example other_example

cargo run --example example_with_options -- --name Sophia Anderson

### packages\sample

cargo run -p sample

cargo run --bin sample 运行指定的二进制文件

## flow

cargo new packages/directory_structure --vcs none

cargo new packages/cargo_example --vcs none

cargo new packages/cargo_test --vcs none

cargo new packages/sample --vcs none
