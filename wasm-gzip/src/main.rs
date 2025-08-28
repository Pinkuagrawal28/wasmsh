use clap::Parser;
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{self, Read, Write};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    input: String,

    /// Decompress file
    #[arg(short, long)]
    decompress: bool,

    /// Output file. If not provided, prints to stdout or creates a default name.
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_path = &args.input;
    let output_path = args.output;

    if args.decompress {
        // Decompress
        let file_in = File::open(input_path)?;
        let mut decoder = GzDecoder::new(file_in);
        let mut decoded_bytes = Vec::new();
        decoder.read_to_end(&mut decoded_bytes)?;

        match output_path {
            Some(path) => {
                let mut file_out = File::create(&path)?;
                file_out.write_all(&decoded_bytes)?;
            }
            None => {
                let default_output_path = if input_path.ends_with(".gz") {
                    input_path.trim_end_matches(".gz").to_string()
                } else {
                    return Err(anyhow!("Cannot determine default output file name for decompression. Please specify --output."));
                };
                let mut file_out = File::create(&default_output_path)?;
                file_out.write_all(&decoded_bytes)?;
            }
        }
    } else {
        // Compress
        let mut file_in = File::open(input_path)?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        let mut buffer = Vec::new();
        file_in.read_to_end(&mut buffer)?;
        encoder.write_all(&buffer)?;
        let compressed_bytes = encoder.finish()?;

        match output_path {
            Some(path) => {
                let mut file_out = File::create(&path)?;
                file_out.write_all(&compressed_bytes)?;
            }
            None => {
                let default_output_path = format!("{}.gz", input_path);
                let mut file_out = File::create(&default_output_path)?;
                file_out.write_all(&compressed_bytes)?;
            }
        }
    }

    Ok(())
}
