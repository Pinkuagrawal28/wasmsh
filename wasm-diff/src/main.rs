use clap::Parser;
use anyhow::{Result, anyhow};
use std::io::{self, BufRead};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the first file
    file1: String,

    /// Path to the second file
    file2: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file1 = File::open(&args.file1)?;
    let file2 = File::open(&args.file2)?;

    let reader1 = io::BufReader::new(file1);
    let reader2 = io::BufReader::new(file2);

    let mut lines1 = reader1.lines();
    let mut lines2 = reader2.lines();

    let mut line_num = 1;

    loop {
        let line1 = lines1.next();
        let line2 = lines2.next();

        match (line1, line2) {
            (Some(l1), Some(l2)) => {
                let l1 = l1?;
                let l2 = l2?;
                if l1 != l2 {
                    println!("< {}\t{}", line_num, l1);
                    println!("> {}\t{}", line_num, l2);
                }
            },
            (Some(l1), None) => {
                println!("< {}\t{}", line_num, l1?);
            },
            (None, Some(l2)) => {
                println!("> {}\t{}", line_num, l2?);
            },
            (None, None) => break,
        }
        line_num += 1;
    }

    Ok(())
}
