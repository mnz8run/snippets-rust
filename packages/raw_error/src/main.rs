mod errors;

fn main() {
    // 使用自定义错误的示例
    match errors::enum_error::trigger_alpha_error() {
        Ok(_) => println!("No Alpha error occurred."),
        Err(e) => eprintln!("Error: {}", e),
    }
    // 使用自定义错误的示例
    match errors::enum_error::trigger_omega_error() {
        Ok(_) => println!("No Omega error occurred."),
        Err(e) => eprintln!("Error: {}", e),
    }
    // 触发 IoError 示例
    match errors::enum_error::trigger_io_error() {
        Ok(_) => println!("No Io error occurred."),
        Err(e) => eprintln!("Error: {}", e),
    }
    // 触发 TheStructError 示例
    match errors::struct_error::trigger_struct_error() {
        Ok(_) => println!("No TheStructError error occurred."),
        Err(e) => eprintln!("Error: {}", e),
    }
    // 触发 TheStructErrorWithMessage 示例
    match errors::struct_error::trigger_struct_message_error() {
        Ok(_) => println!("No TheStructErrorWithMessage error occurred."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
