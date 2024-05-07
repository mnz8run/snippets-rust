fn main() {
    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();

    // 检查是否提供了 --name 参数
    if let Some(index) = args.iter().position(|arg| arg == "--name") {
        // 获取参数值
        if let Some(first_name) = args.get(index + 1) {
            if let Some(last_name) = args.get(index + 2) {
                println!("Hello, {} {}!", first_name, last_name);
                return;
            }
        }
    }

    println!("Usage: cargo run --example example_with_options -- --name <first_name> <last_name>");
}
