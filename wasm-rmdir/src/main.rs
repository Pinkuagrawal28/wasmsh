use anyhow::{Context, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory>...", args[0]);
        std::process::exit(1);
    }

    for dir_path in &args[1..] {
        fs::remove_dir(dir_path)
            .with_context(|| format!("could not remove directory `{}` (it might not be empty)", dir_path))?; // Added context for error message
        println!("Removed directory: {}", dir_path);
    }

    Ok(())
}
