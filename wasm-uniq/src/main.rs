use clap::Parser;
use anyhow::{Result, anyhow};
use std::io::{self, BufRead};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to process. If not provided, reads from stdin.
    file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let reader: Box<dyn BufRead> = match args.file {
        Some(file_path) => Box::new(io::BufReader::new(File::open(&file_path)?)),
        None => Box::new(io::BufReader::new(io::stdin())),
    };

    let mut last_line = String::new();
    let mut first_line = true;

    for line_result in reader.lines() {
        let line = line_result?;
        if first_line || line != last_line {
            println!("{}", line);
            last_line = line;
            first_line = false;
        }
    }

    Ok(())
}
