use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // The first argument is the program name, so we skip it.
    let output = args[1..].join(" ");
    println!("{}", output);
    Ok(())
}
