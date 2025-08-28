use anyhow::{Context, Result};
use std::env;
use std::path::Path;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use fs_extra::file::{copy as copy_file};

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

    if source.is_file() {
        copy_file(source, destination)
            .with_context(|| format!("could not copy file from '{}' to '{}'", source.display(), destination.display()))?;
        println!("Copied file from '{}' to '{}'", source.display(), destination.display());
    } else if source.is_dir() {
        let options = CopyOptions::new(); // Default options, no overwrite, no skip existing
        copy_dir(source, destination, &options)
            .with_context(|| format!("could not copy directory from '{}' to '{}'", source.display(), destination.display()))?;
        println!("Copied directory from '{}' to '{}'", source.display(), destination.display());
    } else {
        bail!("Source '{}' is neither a file nor a directory.", source.display());
    }

    Ok(())
}
