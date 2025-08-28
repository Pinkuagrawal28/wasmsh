# Example: `wasm-touch`

`wasm-touch` is a `touch`-like tool that creates empty files or updates their timestamps.

## Building

```sh
cd wasm-touch
cargo build --target wasm32-wasi --release
```

## Usage

```sh
# Create a new empty file
./wasmsh/target/release/wasmsh run --dir . ./wasm-touch/target/wasm32-wasi/release/wasm-touch.wasm new_file.txt

# Update the timestamp of an existing file
./wasmsh/target/release/wasmsh run --dir . ./wasm-touch/target/wasm32-wasi/release/wasm-touch.wasm existing_file.txt
```
