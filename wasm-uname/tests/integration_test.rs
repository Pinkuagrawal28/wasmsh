use std::process::Command;

#[test]
fn test_uname_s() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-s")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), std::env::consts::OS);
}

#[test]
fn test_uname_m() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-m")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), std::env::consts::ARCH);
}

#[test]
fn test_uname_a() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-a")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    // Check for expected parts of the output, considering placeholders
    assert!(stdout.contains(std::env::consts::OS));
    assert!(stdout.contains("wasm-host"));
    assert!(stdout.contains("1.0.0"));
    assert!(stdout.contains("#1 WASM Kernel"));
    assert!(stdout.contains(std::env::consts::ARCH));
    assert!(stdout.contains("wasm"));
    assert!(stdout.contains("WebAssembly"));
}
