# Example: `wasm-ls`

`wasm-ls` is a simple `ls`-like tool that lists the files in a directory.

## Building

```sh
cd wasm-ls
cargo build --target wasm32-wasi --release
```

## Usage

To run `wasm-ls`, you need to grant it access to the directory you want to list.

```sh
# List the current directory
./wasmsh/target/release/wasmsh run --dir . ./wasm-ls/target/wasm32-wasi/release/wasm-ls.wasm

# List a subdirectory
./wasmsh/target/release/wasmsh run --dir . ./wasm-ls/target/wasm32-wasi/release/wasm-ls.wasm docs
```