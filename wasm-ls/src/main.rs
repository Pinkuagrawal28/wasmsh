use anyhow::Result;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", path.display());
        std::process::exit(1);
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                println!("{}", file_name_str);
            }
        }
    }

    Ok(())
}
