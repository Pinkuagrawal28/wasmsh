# Example: `wasm-head`

`wasm-head` is a `head`-like tool that displays the first few lines of a file.

## Building

```sh
cd wasm-head
cargo build --target wasm32-wasi --release
```

## Usage

```sh
# Create a test file with more than 10 lines
_> for i in $(seq 1 15); do echo "Line $i" >> test.txt; done

# Run wasm-head to get the first 10 lines (default)
./wasmsh/target/release/wasmsh run --dir . ./wasm-head/target/wasm32-wasi/release/wasm-head.wasm test.txt

# Run wasm-head to get the first 3 lines
./wasmsh/target/release/wasmsh run --dir . ./wasm-head/target/wasm32-wasi/release/wasm-head.wasm -n 3 test.txt
```