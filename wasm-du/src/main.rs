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

    let total_size = calculate_size(&path)?;

    println!("{}\t{}", total_size, path.display());

    Ok(())
}

fn calculate_size(path: &PathBuf) -> Result<u64> {
    let mut total_size = 0;

    if path.is_file() {
        total_size += path.metadata()?.len();
    } else if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            total_size += calculate_size(&entry_path)?;
        }
    }
    Ok(total_size)
}
