use std::process::Command;
use std::io::Write;
use std::fs;

#[test]
fn test_sort_file() {
    let input_content = "c\nb\na";
    let expected_output = "a\nb\nc\n";
    let input_file_path = "test_sort_input.txt";
    let mut file = fs::File::create(input_file_path).expect("Could not create test input file");
    file.write_all(input_content.as_bytes()).expect("Could not write to test input file");

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(input_file_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);

    fs::remove_file(input_file_path).expect("Could not remove test input file");
}

#[test]
fn test_sort_stdin() {
    let input_content = "c\nb\na";
    let expected_output = "a\nb\nc\n";

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(input_content.as_bytes()).expect("Failed to write to stdin");
    drop(stdin); // Close stdin to signal EOF

    let output = child.wait_with_output().expect("Failed to wait for command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);
}
