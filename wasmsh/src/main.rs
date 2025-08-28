use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use futures_util::stream::StreamExt;
use oci_distribution::manifest::{WasmConfig, WasmLayer};
use oci_distribution::{Client, Reference};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, copy, Write};
use std::path::PathBuf;
use std::str::FromStr;
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

#[derive(Deserialize, Default, Debug)]
struct Config {
    #[serde(default)]
    aliases: HashMap<String, String>,
}

fn load_config() -> Result<Config> {
    let home_dir = dirs::home_dir().context("could not find home directory")?;
    let config_dir = home_dir.join(".wasmsh");
    std::fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        let mut file = File::create(&config_path)?;
        file.write_all(b"# wasmsh configuration file\n\n# Aliases for commands\n# [aliases]\n# g = \"wasm-grep\"\n")?;
    }

    let config_str = std::fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str).context("failed to parse config file")?;
    Ok(config)
}

/// wasmsh — minimal wasm runner + installer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a local wasm module with optional args
    Run {
        /// Path to the .wasm module
        module: PathBuf,
        /// Directories to allow access to (maps to WASI preopened dirs)
        #[arg(long)]
        dir: Vec<PathBuf>,
        /// Allow network access
        #[arg(long)]
        allow_net: bool,
        /// Passed through to the wasm module
        #[arg(raw = true)]
        args: Vec<String>,
    },

    /// Install a tool from an OCI registry
    Install {
        /// OCI image reference (e.g. ghcr.io/wasmedge/cowsay:latest)
        image_ref: String,
    },

    /// Publish a tool to an OCI registry
    Publish {
        /// Path to the .wasm module to publish
        module: PathBuf,
        /// OCI image reference to publish to (e.g. ghcr.io/your-name/tool:latest)
        image_ref: String,
    },

    /// List installed tools
    List,

    /// Start an interactive shell
    Shell,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config()?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { module, dir, allow_net, args } => {
            let mut wasi_args = vec![module.to_string_lossy().into_owned()];
            wasi_args.extend(args);
            run_module(module, dir, allow_net, wasi_args)
        }
        Commands::Install { image_ref } => install_tool(&image_ref).await,
        Commands::Publish { module, image_ref } => publish_tool(&module, &image_ref).await,
        Commands::List => list_tools(),
        Commands::Shell => start_shell(&config),
    }
}

async fn publish_tool(module_path: &PathBuf, image_ref: &str) -> Result<()> {
    if !module_path.exists() {
        bail!("module path does not exist: {}", module_path.display());
    }

    println!("Publishing {} to {}", module_path.display(), image_ref);

    let reference = Reference::from_str(image_ref)?;

    let wasm_bytes = std::fs::read(module_path)?;

    let wasm_layer = WasmLayer::new(wasm_bytes);
    let wasm_config = WasmConfig::new();

    let mut client = Client::new(Default::default());

    client.push(&reference, &[wasm_layer], wasm_config).await?;

    println!(
        "Successfully published {} to {}",
        module_path.display(),
        image_ref
    );

    Ok(())
}

async fn install_tool(image_ref: &str) -> Result<()> {
    println!("Installing from OCI registry: {}", image_ref);
    let reference = Reference::from_str(image_ref)?;

    let mut client = Client::new(Default::default());
    let mut image_stream = client.pull(
        &reference,
        &["application/vnd.wasm.content.layer.v1+wasm"],
    );

    let mut wasm_bytes = Vec::new();
    while let Some(layer_result) = image_stream.next().await {
        let layer = layer_result?;
        wasm_bytes = layer.data;
        break;
    }

    if wasm_bytes.is_empty() {
        bail!("No wasm layer found in the image.");
    }

    let home_dir = dirs::home_dir().context("could not find home directory")?;
    let install_dir = home_dir.join(".wasmsh/bin");
    std::fs::create_dir_all(&install_dir)?;

    let tool_name = reference.repository().split('/').last().unwrap_or("tool");
    let dest_path = install_dir.join(format!("{}.wasm", tool_name));
    std::fs::write(&dest_path, &wasm_bytes)?;

    println!(
        "Successfully installed '{}' to {:?}",
        image_ref,
        dest_path.display()
    );

    Ok(())
}

fn list_tools() -> Result<()> {
    let home_dir = dirs::home_dir().context("could not find home directory")?;
    let install_dir = home_dir.join(".wasmsh/bin");

    if !install_dir.exists() {
        println!("No tools installed yet.");
        return Ok(());
    }

    println!("Installed tools:");
    for entry in fs::read_dir(install_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "wasm" {
                    if let Some(stem) = path.file_stem() {
                        println!("- {}", stem.to_string_lossy());
                    }
                }
            }
        }
    }

    Ok(())
}

fn start_shell(config: &Config) -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    let history_path = dirs::home_dir()
        .context("could not find home directory")?
        .join(".wasmsh/history.txt");
    if rl.load_history(&history_path).is_err() {
        // No history file yet is fine
    }

    loop {
        let readline = rl.readline("wasmsh> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                let mut parts = input.split_whitespace();
                let command = parts.next().unwrap();
                let command = config
                    .aliases
                    .get(command)
                    .map(|s| s.as_str())
                    .unwrap_or(command);

                let args: Vec<String> = parts.map(String::from).collect();

                if command == "exit" {
                    break;
                }

                let home_dir = dirs::home_dir().context("could not find home directory")?;
                let tool_path = home_dir
                    .join(".wasmsh/bin")
                    .join(format!("{}.wasm", command));

                if tool_path.exists() {
                    let mut wasi_args = vec![tool_path.to_string_lossy().into_owned()];
                    wasi_args.extend(args);
                    match run_module(tool_path.clone(), vec![], false, wasi_args) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Error executing command '{}': {}", command, e),
                    }
                } else {
                    eprintln!("command not found: {}", command);
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl-C, continue
            }
            Err(ReadlineError::Eof) => {
                // Ctrl-D, exit
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(&history_path)?;
    Ok(())
}

fn run_module(module: PathBuf, dirs: Vec<PathBuf>, allow_net: bool, args: Vec<String>) -> Result<()> {
    if !module.exists() {
        bail!("module path does not exist: {}", module.display());
    }

    println!("Running module: {}", module.display());
    println!("Permissions:");
    if dirs.is_empty() {
        println!("  - Filesystem: (none)");
    } else {
        for d in &dirs {
            println!("  - Filesystem (read-write): {}", d.display());
        }
    }
    if !allow_net {
        println!("  - Network: (none)");
    } else {
        println!("  - Network: all");
    }

    print!("Proceed? [y/N] ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim().to_lowercase() != "y" {
        println!("Aborted.");
        return Ok(());
    }

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let mut wasi_builder = WasiCtxBuilder::new().inherit_stdio();

    let wasm_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    wasi_builder = wasi_builder.args(&wasm_args)?;

    if allow_net {
        wasi_builder = wasi_builder.inherit_network();
    }

    for d in dirs {
        let preopened_dir =
            wasmtime_wasi::sync::Dir::open_ambient_dir(&d, wasmtime_wasi::sync::ambient_authority())
                .with_context(|| format!("failed to open directory '{}'", d.display()))?;
        let guest_path = d.file_name().unwrap_or(d.as_os_str());
        wasi_builder = wasi_builder.preopened_dir(Box::new(preopened_dir), guest_path)?;
    }

    let mut store = Store::new(&engine, wasi_builder.build());

    let module = Module::from_file(&engine, &module)
        .with_context(|| format!("failed to load wasm module '{}'", module.display()))?;

    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    Ok(())
}
