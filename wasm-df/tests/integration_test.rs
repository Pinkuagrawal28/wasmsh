use std::process::Command;
use std::io::Write;
use std::fs;

#[test]
fn test_df_basic() {
    let test_dir = "test_df_dir";
    let file1 = format!("{}/file1.txt", test_dir);

    // Create temporary directory and file
    fs::create_dir_all(test_dir).expect("Failed to create test directory");
    let mut file = fs::File::create(&file1).expect("Failed to create file1");
    file.write_all(b"hello").expect("Failed to write to file1"); // Write some content to get a size

    // Run wasm-df
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(test_dir)
        .output()
        .expect("Failed to execute command");

    // Assert that the command ran successfully
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Check for expected headers and some content, given the simplified nature of wasm-df
    assert!(stdout.contains("Filesystem"));
    assert!(stdout.contains("Size"));
    assert!(stdout.contains("Used"));
    assert!(stdout.contains("Avail"));
    assert!(stdout.contains("Use%"));
    assert!(stdout.contains("Mounted on"));
    assert!(stdout.contains("wasmfs"));
    assert!(stdout.contains(test_dir));

    // Clean up
    fs::remove_dir_all(test_dir).expect("Failed to remove test directory");
}
