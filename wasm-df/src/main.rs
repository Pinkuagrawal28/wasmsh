use clap::Parser;
use anyhow::{Result, anyhow};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to analyze disk usage for. Defaults to current directory.
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let path = args.path.unwrap_or_else(|| PathBuf::from("."));

    if !path.exists() {
        return Err(anyhow!("Path does not exist: {}", path.display()));
    }

    let total_size = calculate_dir_size(&path)?;

    println!("Filesystem    Size  Used Avail Use% Mounted on");
    // Placeholder values for a WASM environment
    // The 'Size', 'Used', 'Avail' will reflect the sum of file sizes in the accessible virtual filesystem
    // 'Mounted on' will be the path analyzed
    println!("wasmfs        {}K {}K {}K   {}% {}",
        total_size / 1024, // Size in KB
        total_size / 1024, // Used in KB (assuming all space is used by files found)
        0, // Avail in KB (no concept of free space in this simplified model)
        100, // Use% (always 100% in this simplified model)
        path.display()
    );

    Ok(())
}

fn calculate_dir_size(path: &PathBuf) -> Result<u64> {
    let mut total_size = 0;

    if path.is_file() {
        total_size += path.metadata()?.len();
    } else if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() {
                total_size += entry_path.metadata()?.len();
            } else if entry_path.is_dir() {
                total_size += calculate_dir_size(&entry_path)?;
            }
        }
    }
    Ok(total_size)
}
