fn main() {
    println!("Hello, test!");
}

/**
 * 一个单元测试
 * #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，而在运行 cargo build 时不这么做。
 * #[test] 注解告诉 Rust 这是一个测试函数。
 */
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn unit_run_main() {
        main();
    }
}
