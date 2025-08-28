use clap::Parser;
use anyhow::{Result, anyhow};
use std::io::{self, BufRead};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to sort. If not provided, reads from stdin.
    file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut lines = Vec::new();

    match args.file {
        Some(file_path) => {
            let file = File::open(&file_path)?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                lines.push(line?);
            }
        }
        None => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                lines.push(line?);
            }
        }
    }

    lines.sort();

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
