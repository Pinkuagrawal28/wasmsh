use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source> <destination>", args[0]);
        std::process::exit(1);
    }

    let source = Path::new(&args[1]);
    let destination = Path::new(&args[2]);

    if !source.exists() {
        bail!("Source '{}' does not exist.", source.display());
    }

    fs::rename(source, destination)
        .with_context(|| format!("could not move/rename from '{}' to '{}'", source.display(), destination.display()))?;

    println!("Moved/renamed '{}' to '{}'", source.display(), destination.display());

    Ok(())
}
