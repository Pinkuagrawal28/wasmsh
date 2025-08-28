use anyhow::{Context, Result};
use std::env;
use std::fs::File;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>...", args[0]);
        std::process::exit(1);
    }

    for file_path in &args[1..] {
        File::create(file_path)
            .with_context(|| format!("could not touch file `{}`", file_path))?;
        println!("Touched file: {}", file_path);
    }

    Ok(())
}
