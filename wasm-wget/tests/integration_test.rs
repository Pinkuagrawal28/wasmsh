use std::process::Command;
use std::fs;
use std::io::Read;

#[tokio::test]
async fn test_wget_download_to_file() {
    let url = "https://httpbin.org/anything";
    let output_file = "wget_test_output.json";

    // Run wasm-wget to download to a file
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(url)
        .arg("--output")
        .arg(output_file)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(fs::metadata(output_file).is_ok());

    let mut file = fs::File::open(output_file).expect("Failed to open downloaded file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read downloaded file");

    // Basic check that it contains some expected content from httpbin
    assert!(contents.contains("\"url\": \"https://httpbin.org/anything\""));

    fs::remove_file(output_file).expect("Failed to remove downloaded file");
}

#[tokio::test]
async fn test_wget_download_to_stdout() {
    let url = "https://httpbin.org/get";

    // Run wasm-wget to download to stdout
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(url)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Basic check that it contains some expected content from httpbin
    assert!(stdout.contains("\"url\": \"https://httpbin.org/get\""));
}
