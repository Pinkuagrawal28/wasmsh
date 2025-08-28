use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, anyhow};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The starting path for the search
    path: PathBuf,

    /// Optional pattern to search for (e.g., "*.txt")
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.path.exists() {
        return Err(anyhow!("Path does not exist: {}", args.path.display()));
    }

    find_files(&args.path, args.name.as_deref())?;

    Ok(())
}

fn find_files(path: &PathBuf, pattern: Option<&str>) -> Result<()> {
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find_files(&path, pattern)?;
            } else {
                if let Some(p) = pattern {
                    if path.file_name().and_then(|n| n.to_str()).map_or(false, |name| name.contains(p)) {
                        println!("{}", path.display());
                    }
                } else {
                    println!("{}", path.display());
                }
            }
        }
    } else {
        if let Some(p) = pattern {
            if path.file_name().and_then(|n| n.to_str()).map_or(false, |name| name.contains(p)) {
                println!("{}", path.display());
            }
        } else {
            println!("{}", path.display());
        }
    }
    Ok(())
}
