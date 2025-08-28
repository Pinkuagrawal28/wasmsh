# Example: `wasm-wc`

`wasm-wc` is a `wc`-like tool that counts the number of lines, words, and bytes in a file.

## Building

```sh
cd wasm-wc
cargo build --target wasm32-wasi --release
```

## Usage

```sh
# Create a test file
_> echo "Hello, wasm-wc!\nThis is a test." > test.txt

# Run wasm-wc with wasmsh
./wasmsh/target/release/wasmsh run --dir . ./wasm-wc/target/wasm32-wasi/release/wasm-wc.wasm test.txt
```