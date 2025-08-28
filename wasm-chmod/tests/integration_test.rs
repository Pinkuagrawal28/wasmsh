use std::process::Command;
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[test]
fn test_chmod_basic() {
    let test_file = "test_chmod_file.txt";
    fs::File::create(test_file).expect("Failed to create test file");

    // Set permissions to 755
    let mode = "755";
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(mode)
        .arg(test_file)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let metadata = fs::metadata(test_file).expect("Failed to get file metadata");
    // Check if the permissions are approximately 755 (some bits might be set by default)
    // We check for the lower 9 bits (rwx for owner, group, others)
    assert_eq!(metadata.permissions().mode() & 0o777, 0o755);

    fs::remove_file(test_file).expect("Failed to remove test file");
}

#[test]
fn test_chmod_invalid_mode() {
    let test_file = "test_chmod_invalid_file.txt";
    fs::File::create(test_file).expect("Failed to create test file");

    // Try to set invalid permissions
    let mode = "abc";
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(mode)
        .arg(test_file)
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success()); // Should fail
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid mode"));

    fs::remove_file(test_file).expect("Failed to remove test file");
}
