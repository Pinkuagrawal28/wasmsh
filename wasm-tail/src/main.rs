use anyhow::{Context, Result};
use pico_args::Arguments;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DEFAULT_LINES: usize = 10;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    let n_lines: usize = args
        .opt_value_from_str("-n")?
        .unwrap_or(DEFAULT_LINES);

    let file_path: String = args.free_from_str()?;

    let file = File::open(&file_path)
        .with_context(|| format!("could not open file `{}`", file_path))?;
    let reader = BufReader::new(file);

    let mut buffer: VecDeque<String> = VecDeque::with_capacity(n_lines);

    for line in reader.lines() {
        if buffer.len() == n_lines {
            buffer.pop_front();
        }
        buffer.push_back(line?);
    }

    for line in buffer {
        println!("{}", line);
    }

    Ok(())
}
