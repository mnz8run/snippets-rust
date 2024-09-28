use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct TheStructError {
    holding_error: Option<Box<dyn Error + 'static>>,
}

impl TheStructError {
    // 构造函数
    pub fn new(error: Option<Box<dyn Error + 'static>>) -> Self {
        TheStructError { holding_error: error }
    }
}

impl fmt::Display for TheStructError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TheStructError occurred: {}",
            self.holding_error.as_ref().map_or("No underlying error".to_string(), |e| e.to_string())
        )
    }
}

impl Error for TheStructError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.holding_error.as_ref().map(|s| s.as_ref())
    }
}

// 实现 From trait
impl From<std::io::Error> for TheStructError {
    fn from(err: std::io::Error) -> TheStructError {
        TheStructError::new(Some(Box::new(err)))
    }
}

pub fn trigger_struct_io() -> Result<(), TheStructError> {
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "IO error occurred");
    Err(TheStructError::from(io_error)) // 使用 From trait 进行转换
}

// ------------------------------------------------------------

#[derive(Debug)]
pub enum TheEnumError {
    NotFound(String),
    InvalidInput(String),
}

impl fmt::Display for TheEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TheEnumError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            TheEnumError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
        }
    }
}

impl Error for TheEnumError {}

// 实现 From trait
impl From<ParseIntError> for TheEnumError {
    fn from(err: ParseIntError) -> TheEnumError {
        TheEnumError::InvalidInput(err.to_string())
    }
}

// 使用自定义错误
fn parse_input(input: &str) -> Result<i32, TheEnumError> {
    // 这里会自动转换
    let num: i32 = input.parse()?;
    Ok(num)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_trigger_struct_io() {
        match trigger_struct_io() {
            Ok(_) => println!("没触发 error"),
            Err(e) => {
                eprintln!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("Caused by: {}", source);
                }
            }
        }

        match parse_input("abc") {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => println!("Error: {}", e),
        }
    }
}
