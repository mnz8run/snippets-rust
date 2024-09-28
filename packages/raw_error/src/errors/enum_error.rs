use std::fmt;

// 定义自定义错误类型 使用枚举
#[derive(Debug)]
pub enum TheEnumError {
    AlphaError(String),
    OmegaError(String),
    IoError(std::io::Error),
}

// 实现 fmt::Display 特征
impl fmt::Display for TheEnumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TheEnumError::IoError(err) => write!(f, "TheEnumError IO: {}", err),
            TheEnumError::AlphaError(msg) => write!(f, "TheEnumError Alpha: {}", msg),
            TheEnumError::OmegaError(msg) => write!(f, "TheEnumError Omega: {}", msg),
        }
    }
}

// 实现 std::error::Error 特征
// 为自定义错误类型实现 std::error::Error 是一种良好的实践，可以让你的代码与 Rust 生态系统更好地集成。
impl std::error::Error for TheEnumError {}

// 模拟触发 AlphaError 的函数
pub fn trigger_alpha_error() -> Result<(), TheEnumError> {
    Err(TheEnumError::AlphaError("An Alpha error occurred!".into()))
}

// 模拟触发 OmegaError 的函数
pub fn trigger_omega_error() -> Result<(), TheEnumError> {
    Err(TheEnumError::OmegaError("An Omega error occurred!".into()))
}

// 模拟触发 IoError 的函数
pub fn trigger_io_error() -> Result<(), TheEnumError> {
    Err(TheEnumError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "手动创建一个 io Error")))
}
