use anyhow::Result;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let metadata = fs::metadata(file_path)?;
    let bytes = metadata.len();

    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut words = 0;

    for line in reader.lines() {
        let line = line?;
        lines += 1;
        words += line.split_whitespace().count();
    }

    println!(" {}  {} {} {}", lines, words, bytes, file_path);

    Ok(())
}
