mod shared;

/**
 * 使用共享的代码一个集成测试
 */
#[test]
fn shared_setup() {
    assert_eq!(shared::setup(), "Shared, setup!");
}
