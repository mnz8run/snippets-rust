// 在集成测试之间共享一些代码
pub fn setup() -> &'static str {
    "Shared, setup!"
}

// some setup code, like creating required files/directories, starting
// servers, etc.
