# `wasm-hostname`

`wasm-hostname` is a WebAssembly version of the `hostname` command, used to display the system's hostname. In a WebAssembly environment, the returned hostname might be a generic value provided by the WASM runtime or a placeholder, as direct access to the host system's hostname is typically restricted.

## Usage

```bash
wasm-hostname
```

## Examples

Print the hostname:

```bash
wasm-hostname
```
