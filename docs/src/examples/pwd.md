# Example: `wasm-pwd`

`wasm-pwd` is a `pwd`-like tool that prints the current working directory.

## Building

```sh
cd wasm-pwd
cargo build --target wasm32-wasi --release
```

## Usage

```sh
./wasmsh/target/release/wasmsh run --dir . ./wasm-pwd/target/wasm32-wasi/release/wasm-pwd.wasm
```