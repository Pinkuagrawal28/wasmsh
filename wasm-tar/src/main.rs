use anyhow::Result;
use std::env;
use std::fs::File;
use tar::Builder;
use tar::Archive;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  {} c <archive.tar> <file1> [file2]...", args[0]);
        eprintln!("  {} x <archive.tar>", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    let archive_path = &args[2];

    match mode.as_str() {
        "c" => {
            if args.len() < 4 {
                eprintln!("Usage: {} c <archive.tar> <file1> [file2]...", args[0]);
                std::process::exit(1);
            }
            let file_paths = &args[3..];
            let file = File::create(archive_path)?;
            let mut builder = Builder::new(file);
            for path in file_paths {
                builder.append_path(path)?;
            }
            builder.finish()?;
            println!("Created archive: {}", archive_path);
        }
        "x" => {
            let file = File::open(archive_path)?;
            let mut archive = Archive::new(file);
            archive.unpack(".")?; // Unpack to current directory
            println!("Extracted archive: {}", archive_path);
        }
        _ => {
            eprintln!("Invalid mode: {}. Use 'c' to create or 'x' to extract.", mode);
            std::process::exit(1);
        }
    }

    Ok(())
}
