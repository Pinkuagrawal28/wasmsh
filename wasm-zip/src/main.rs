use anyhow::Result;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::copy;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage:");
        eprintln!("  {} c <input> <output.gz>", args[0]);
        eprintln!("  {} d <input.gz> <output>", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];

    match mode.as_str() {
        "c" => {
            let mut input_file = File::open(input_path)?;
            let output_file = File::create(output_path)?;
            let mut encoder = GzEncoder::new(output_file, Compression::default());
            copy(&mut input_file, &mut encoder)?;
            encoder.finish()?;
            println!("Compressed {} to {}", input_path, output_path);
        }
        "d" => {
            let input_file = File::open(input_path)?;
            let mut decoder = GzDecoder::new(input_file);
            let mut output_file = File::create(output_path)?;
            copy(&mut decoder, &mut output_file)?;
            println!("Decompressed {} to {}", input_path, output_path);
        }
        _ => {
            eprintln!("Invalid mode: {}. Use 'c' to compress or 'd' to decompress.", mode);
            std::process::exit(1);
        }
    }

    Ok(())
}
