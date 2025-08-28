use clap::Parser;
use anyhow::{Result, anyhow};
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The octal mode (e.g., "755")
    mode: String,

    /// The file to change permissions for
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mode = u32::from_str_radix(&args.mode, 8)
        .map_err(|_| anyhow!("Invalid mode: {}", args.mode))?;

    let mut permissions = fs::metadata(&args.file)?.permissions();
    permissions.set_mode(mode);

    fs::set_permissions(&args.file, permissions)?;

    Ok(())
}
