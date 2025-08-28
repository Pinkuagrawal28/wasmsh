use std::process::Command;
use std::io::Write;
use std::fs;
use std::path::Path;

#[test]
fn test_find_basic() {
    let test_dir = "test_find_dir";
    let file1 = format!("{}/file1.txt", test_dir);
    let subdir = format!("{}/subdir", test_dir);
    let file2 = format!("{}/subdir/file2.log", test_dir);

    // Create temporary directory and files
    fs::create_dir_all(&subdir).expect("Failed to create test directory");
    fs::File::create(&file1).expect("Failed to create file1");
    fs::File::create(&file2).expect("Failed to create file2");

    // Run wasm-find
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(test_dir)
        .output()
        .expect("Failed to execute command");

    // Assert that the command ran successfully
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(&file1));
    assert!(stdout.contains(&file2));
    assert!(stdout.contains(&subdir)); // Directories are also listed by find

    // Clean up
    fs::remove_dir_all(test_dir).expect("Failed to remove test directory");
}

#[test]
fn test_find_name_filter() {
    let test_dir = "test_find_name_dir";
    let file1 = format!("{}/document.txt", test_dir);
    let file2 = format!("{}/image.png", test_dir);

    // Create temporary directory and files
    fs::create_dir_all(test_dir).expect("Failed to create test directory");
    fs::File::create(&file1).expect("Failed to create file1");
    fs::File::create(&file2).expect("Failed to create file2");

    // Run wasm-find with name filter
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(test_dir)
        .arg("--name")
        .arg("txt")
        .output()
        .expect("Failed to execute command");

    // Assert that the command ran successfully
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(&file1));
    assert!(!stdout.contains(&file2));

    // Clean up
    fs::remove_dir_all(test_dir).expect("Failed to remove test directory");
}
