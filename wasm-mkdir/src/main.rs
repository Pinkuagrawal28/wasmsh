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
        fs::create_dir_all(dir_path)
            .with_context(|| format!("could not create directory `{}`", dir_path))?;
        println!("Created directory: {}", dir_path);
    }

    Ok(())
}
