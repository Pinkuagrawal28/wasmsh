# Example: `wasm-echo`

`wasm-echo` is a simple tool that prints its arguments to the console. It's useful for testing argument passing.

## Building

```sh
cd wasm-echo
cargo build --target wasm32-wasi --release
```

## Usage

```sh
./wasmsh/target/release/wasmsh run ./wasm-echo/target/wasm32-wasi/release/wasm-echo.wasm Hello, wasmsh!
```