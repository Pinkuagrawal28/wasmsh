use anyhow::{Context, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <pattern> <file>", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let file_path = &args[2];

    let content = fs::read_to_string(file_path)
        .with_context(|| format!("could not read file `{}`", file_path))?;

    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
