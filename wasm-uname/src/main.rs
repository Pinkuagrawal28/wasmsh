use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Print all information
    #[arg(short, long)]
    all: bool,

    /// Print the kernel name
    #[arg(short, long)]
    kernel_name: bool,

    /// Print the network node hostname
    #[arg(short, long)]
    nodename: bool,

    /// Print the kernel release
    #[arg(short, long)]
    kernel_release: bool,

    /// Print the kernel version
    #[arg(short, long)]
    kernel_version: bool,

    /// Print the machine hardware name
    #[arg(short, long)]
    machine: bool,

    /// Print the processor type
    #[arg(short, long)]
    processor: bool,

    /// Print the hardware platform
    #[arg(short, long)]
    hardware_platform: bool,

    /// Print the operating system
    #[arg(short, long)]
    operating_system: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut output = Vec::new();

    let kernel_name = std::env::consts::OS;
    let nodename = "wasm-host"; // Placeholder for WASM environment
    let kernel_release = "1.0.0"; // Placeholder for WASM environment
    let kernel_version = "#1 WASM Kernel"; // Placeholder for WASM environment
    let machine = std::env::consts::ARCH;
    let processor = std::env::consts::ARCH; // Often same as machine
    let hardware_platform = "wasm"; // Placeholder for WASM environment
    let operating_system = "WebAssembly"; // Placeholder for WASM environment

    if args.all || (!args.kernel_name && !args.nodename && !args.kernel_release && !args.kernel_version && !args.machine && !args.processor && !args.hardware_platform && !args.operating_system) {
        // Default behavior if no flags are provided is to print kernel name
        if !args.all && !args.kernel_name && !args.nodename && !args.kernel_release && !args.kernel_version && !args.machine && !args.processor && !args.hardware_platform && !args.operating_system {
            output.push(kernel_name);
        } else {
            // If -a is used, print all
            output.push(kernel_name);
            output.push(nodename);
            output.push(kernel_release);
            output.push(kernel_version);
            output.push(machine);
            output.push(processor);
            output.push(hardware_platform);
            output.push(operating_system);
        }
    } else {
        if args.kernel_name { output.push(kernel_name); }
        if args.nodename { output.push(nodename); }
        if args.kernel_release { output.push(kernel_release); }
        if args.kernel_version { output.push(kernel_version); }
        if args.machine { output.push(machine); }
        if args.processor { output.push(processor); }
        if args.hardware_platform { output.push(hardware_platform); }
        if args.operating_system { output.push(operating_system); }
    }

    println!("{}", output.join(" "));

    Ok(())
}
