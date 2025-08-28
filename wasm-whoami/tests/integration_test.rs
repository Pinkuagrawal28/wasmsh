use std::process::Command;

#[test]
fn test_whoami_basic() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "wasm_user");
}
