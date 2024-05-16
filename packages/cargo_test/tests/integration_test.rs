use std::process::Command;

/**
 * 一个集成测试
 */
#[test]
fn integration_run_main() {
    let output = Command::new("cargo")
        .arg("run")
        // your_binary_argument
        .arg("cargo_test")
        // .arg("--")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello, test!"));
}
