# `wasm-df`

`wasm-df` is a WebAssembly version of the `df` command, used to report disk space usage. In a WebAssembly environment, this command reports on the usage of the virtual filesystem accessible to the WASM module, rather than the host system's physical disk.

## Usage

```bash
wasm-df [PATH]
```

- `[PATH]`: (Optional) The path to analyze disk usage for. Defaults to the current directory.

## Examples

Report disk space usage for the current directory:

```bash
wasm-df .
```

Report disk space usage for a specific directory:

```bash
wasm-df /path/to/directory
```
