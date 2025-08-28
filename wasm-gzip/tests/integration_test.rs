use std::process::Command;
use std::io::{self, Write, Read};
use std::fs;
use flate2::write::GzEncoder;
use flate2::Compression;
use flate2::read::GzDecoder;

#[test]
fn test_gzip_compress_decompress() {
    let original_content = "This is some test content to be compressed and decompressed.";
    let input_file = "test_compress_input.txt";
    let compressed_file = "test_compress_input.txt.gz";
    let decompressed_file = "test_decompressed_output.txt";

    // Create original input file
    fs::File::create(input_file).unwrap().write_all(original_content.as_bytes()).unwrap();

    // 1. Test Compression
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(input_file)
        .output()
        .expect("Failed to execute compression command");

    assert!(output.status.success());
    assert!(fs::metadata(compressed_file).is_ok());

    // 2. Test Decompression
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--decompress")
        .arg(compressed_file)
        .arg("--output")
        .arg(decompressed_file)
        .output()
        .expect("Failed to execute decompression command");

    assert!(output.status.success());
    assert!(fs::metadata(decompressed_file).is_ok());

    let mut file = fs::File::open(decompressed_file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    assert_eq!(content, original_content);

    // Clean up
    fs::remove_file(input_file).unwrap();
    fs::remove_file(compressed_file).unwrap();
    fs::remove_file(decompressed_file).unwrap();
}

#[test]
fn test_gzip_decompress_default_output() {
    let original_content = "Another test content.";
    let input_file_name = "test_decompress_default.txt";
    let compressed_file_name = format!("{}.gz", input_file_name);

    // Manually compress content to create a .gz file
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(original_content.as_bytes()).unwrap();
    let compressed_bytes = encoder.finish().unwrap();
    fs::File::create(&compressed_file_name).unwrap().write_all(&compressed_bytes).unwrap();

    // Test Decompression with default output name
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--decompress")
        .arg(&compressed_file_name)
        .output()
        .expect("Failed to execute decompression command");

    assert!(output.status.success());
    assert!(fs::metadata(&input_file_name).is_ok()); // Check if default output file was created

    let mut file = fs::File::open(&input_file_name).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    assert_eq!(content, original_content);

    // Clean up
    fs::remove_file(&compressed_file_name).unwrap();
    fs::remove_file(&input_file_name).unwrap();
}
