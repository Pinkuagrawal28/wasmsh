use clap::Parser;
use anyhow::{Result, anyhow};
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL to download
    url: String,

    /// Output file name. If not provided, prints to stdout.
    #[arg(short, long)]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let response = reqwest::get(&args.url).await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to download: {}", response.status()));
    }

    let bytes = response.bytes().await?;

    match args.output {
        Some(output_path) => {
            let mut file = std::fs::File::create(&output_path)?;
            file.write_all(&bytes)?;
            println!("Downloaded to {}", output_path);
        }
        None => {
            std::io::stdout().write_all(&bytes)?;
        }
    }

    Ok(())
}
