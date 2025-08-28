# Example: `wasm-mkdir`

`wasm-mkdir` is a `mkdir`-like tool that creates new directories. It supports creating parent directories if they don't exist.

## Building

```sh
cd wasm-mkdir
cargo build --target wasm32-wasi --release
```

## Usage

```sh
./wasmsh/target/release/wasmsh run --dir . ./wasm-mkdir/target/wasm32-wasi/release/wasm-mkdir.wasm new_directory/subdir
```