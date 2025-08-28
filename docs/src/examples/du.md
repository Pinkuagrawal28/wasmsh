# `wasm-du`

`wasm-du` is a WebAssembly version of the `du` command, used to estimate file space usage. In a WebAssembly environment, this command reports on the usage of the virtual filesystem accessible to the WASM module, rather than the host system's physical disk.

## Usage

```bash
wasm-du [PATH]
```

- `[PATH]`: (Optional) The path to analyze disk usage for. Defaults to the current directory.

## Examples

Estimate disk space usage for the current directory:

```bash
wasm-du .
```

Estimate disk space usage for a specific directory:

```bash
wasm-du /path/to/directory
```
