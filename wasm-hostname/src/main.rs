use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() -> Result<()> {
    match gethostname::gethostname().into_string() {
        Ok(hostname) => println!("{}", hostname),
        Err(_) => println!("wasm-host"), // Fallback for WASM environment
    }
    Ok(())
}
