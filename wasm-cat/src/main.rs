use anyhow::Result;
use std::env;
use std::fs;
use std::io::{self, Read};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file1> [file2]...", args[0]);
        std::process::exit(1);
    }

    for file_path in &args[1..] {
        let mut file = fs::File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        print!("{}", contents);
    }

    Ok(())
}
