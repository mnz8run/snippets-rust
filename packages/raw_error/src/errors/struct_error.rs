use std::fmt;

// 定义自定义错误类型 使用结构体
#[derive(Debug)]
pub struct TheStructError {}

// 实现 fmt::Display 特征
impl fmt::Display for TheStructError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TheStructError is here!")
    }
}

// 实现 std::error::Error 特征
impl std::error::Error for TheStructError {}

// 模拟触发 TheStructError 的函数
pub fn trigger_struct_error() -> Result<(), TheStructError> {
    Err(TheStructError {})
}

// 定义自定义错误类型 使用结构体
#[derive(Debug)]
pub struct TheStructErrorWithMessage {
    message: String,
}

// 实现 fmt::Display 特征
impl fmt::Display for TheStructErrorWithMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TheStructErrorWithMessage: {}", self.message)
    }
}

// 实现 std::error::Error 特征
impl std::error::Error for TheStructErrorWithMessage {}

// 模拟触发 TheStructErrorWithMessage 的函数
pub fn trigger_struct_message_error() -> Result<(), TheStructErrorWithMessage> {
    Err(TheStructErrorWithMessage {
        message: "An StructErrorWithMessage error occurred!".to_string(),
    })
}
