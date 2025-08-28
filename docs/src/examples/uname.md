# `wasm-uname`

`wasm-uname` is a WebAssembly version of the `uname` command, used to display system information.

## Usage

```bash
wasm-uname [OPTIONS]
```

## Options

- `-a`, `--all`: Print all information.
- `-s`, `--kernel-name`: Print the kernel name.
- `-n`, `--nodename`: Print the network node hostname.
- `-r`, `--kernel-release`: Print the kernel release.
- `-v`, `--kernel-version`: Print the kernel version.
- `-m`, `--machine`: Print the machine hardware name.
- `-p`, `--processor`: Print the processor type.
- `-i`, `--hardware-platform`: Print the hardware platform.
- `-o`, `--operating-system`: Print the operating system.

If no options are specified, `wasm-uname` prints the kernel name.

## Examples

Print the kernel name:

```bash
wasm-uname
```

Print all system information:

```bash
wasm-uname -a
```
