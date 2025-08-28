use anyhow::{Context, Result};
use pico_args::Arguments;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    let symbolic: bool = args.contains("-s");

    let target_path: String = args.free_from_str()?;
    let link_path: String = args.free_from_str()?;

    let target = Path::new(&target_path);
    let link = Path::new(&link_path);

    if !target.exists() {
        bail!("Target '{}' does not exist.", target.display());
    }

    if symbolic {
        fs::symlink(target, link)
            .with_context(|| format!("could not create symbolic link from '{}' to '{}'", target.display(), link.display()))?;
    } else {
        fs::hard_link(target, link)
            .with_context(|| format!("could not create hard link from '{}' to '{}'", target.display(), link.display()))?;
    }

    println!("Created link from '{}' to '{}'", target.display(), link.display());

    Ok(())
}
