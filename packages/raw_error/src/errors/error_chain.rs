use std::fmt;

#[derive(Debug)]
pub enum TheEnumError {
    ErrorVariant1,
    ErrorVariant2,
}

impl fmt::Display for TheEnumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 每个变体都是相同的错误信息，
        // 如果希望不同的错误信息，使用 match 匹配。
        write!(f, "eeeee")
    }
}

impl std::error::Error for TheEnumError {}

pub fn trigger_enum() -> Result<(), TheEnumError> {
    Err(TheEnumError::ErrorVariant1)
}

pub fn trigger_enum_2() -> Result<(), TheEnumError> {
    Err(TheEnumError::ErrorVariant2)
}

// ----------------------------------------------------------

#[derive(Debug)]
pub struct TheStructError {
    holding_error: Option<Box<dyn std::error::Error + 'static>>,
}

impl TheStructError {
    // 构造函数
    pub fn new(error: Option<Box<dyn std::error::Error + 'static>>) -> Self {
        TheStructError { holding_error: error }
    }
}

impl fmt::Display for TheStructError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sssss")
    }
}

impl std::error::Error for TheStructError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.holding_error.as_ref().map(|s| s.as_ref())
    }
}

pub fn trigger_struct() -> Result<(), TheStructError> {
    // 这里传入 None 或其他错误
    Err(TheStructError::new(None))
}

pub fn trigger_struct_io() -> Result<(), TheStructError> {
    // 模拟一个错误
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "手动创建一个 IO Error");
    Err(TheStructError::new(Some(Box::new(io_error))))
}

#[cfg(test)]
mod unit_tests {
    // 调用 trait 的方法（如 source()），而没有在作用域中引入该 trait，编译器会报错，因为它找不到对应的方法。
    use std::error::Error;
    // 即使你的 TheStructError 实现了 std::error::Error trait，
    // 但在测试模块中，编译器依然需要知道你希望使用这个 trait 的方法。
    // 因此，必须通过 use std::error::Error; 显式引入。

    #[test]
    fn unit_run_main() {
        match super::trigger_enum() {
            Ok(_) => println!("没触发 error"),
            Err(e) => eprintln!("Error: {}", e),
        }
        match super::trigger_enum_2() {
            Ok(_) => println!("没触发 error"),
            Err(e) => eprintln!("Error: {}", e),
        }
        match super::trigger_struct() {
            Ok(_) => println!("没触发 error"),
            Err(e) => {
                eprintln!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("Caused by: {}", source);
                } else {
                    println!("Caused by: None");
                }
            }
        }
        match super::trigger_struct_io() {
            Ok(_) => println!("没触发 error"),
            Err(e) => {
                eprintln!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("Caused by: {}", source);
                } else {
                    println!("Caused by: None");
                }
            }
        }
    }
}
