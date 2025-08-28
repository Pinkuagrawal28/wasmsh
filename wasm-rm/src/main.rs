use anyhow::{Context, Result};
use std::env;
use std::path::Path;
use fs_extra::dir::{remove as remove_dir};
use fs_extra::file::{remove as remove_file};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_or_directory>...", args[0]);
        std::process::exit(1);
    }

    for path_str in &args[1..] {
        let path = Path::new(path_str);

        if !path.exists() {
            eprintln!("Warning: '{}' does not exist.", path.display());
            continue;
        }

        if path.is_file() {
            remove_file(path)
                .with_context(|| format!("could not remove file '{}'", path.display()))?;
            println!("Removed file: '{}'", path.display());
        } else if path.is_dir() {
            remove_dir(path)
                .with_context(|| format!("could not remove directory '{}'", path.display()))?;
            println!("Removed directory: '{}'", path.display());
        } else {
            eprintln!("Warning: '{}' is neither a file nor a directory, skipping.", path.display());
        }
    }

    Ok(())
}
