use std::process::Command;
use std::io::Write;
use std::fs;

#[test]
fn test_du_basic() {
    let test_dir = "test_du_dir";
    let file1 = format!("{}/file1.txt", test_dir);
    let subdir = format!("{}/subdir", test_dir);
    let file2 = format!("{}/subdir/file2.log", test_dir);

    // Create temporary directory and files with known sizes
    fs::create_dir_all(&subdir).expect("Failed to create test directory");
    let mut f1 = fs::File::create(&file1).expect("Failed to create file1");
    f1.write_all(b"12345").expect("Failed to write to file1"); // 5 bytes
    let mut f2 = fs::File::create(&file2).expect("Failed to create file2");
    f2.write_all(b"abcdefghij").expect("Failed to write to file2"); // 10 bytes

    // Run wasm-du
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(test_dir)
        .output()
        .expect("Failed to execute command");

    // Assert that the command ran successfully
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Expected total size is 15 bytes (5 + 10)
    assert!(stdout.contains("15"));
    assert!(stdout.contains(test_dir));

    // Clean up
    fs::remove_dir_all(test_dir).expect("Failed to remove test directory");
}
