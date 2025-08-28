# Example: `wasm-rand`

`wasm-rand` is a tool that generates random numbers. It demonstrates how a wasm module can access the host's source of entropy.

## Building

```sh
cd wasm-rand
cargo build --target wasm32-wasi --release
```

## Usage

```sh
./wasmsh/target/release/wasmsh run ./wasm-rand/target/wasm32-wasi/release/wasm-rand.wasm
```
This will print a random 32-bit unsigned integer.