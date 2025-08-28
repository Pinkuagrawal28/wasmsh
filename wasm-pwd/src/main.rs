use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    println!("{}", current_dir.display());
    Ok(())
}
