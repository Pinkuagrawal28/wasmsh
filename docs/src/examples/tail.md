# Example: `wasm-tail`

`wasm-tail` is a `tail`-like tool that displays the last few lines of a file. It uses a circular buffer for efficient processing.

## Building

```sh
cd wasm-tail
cargo build --target wasm32-wasi --release
```

## Usage

```sh
# Create a test file with more than 10 lines
_> for i in $(seq 1 15); do echo "Line $i" >> test.txt; done

# Run wasm-tail to get the last 10 lines (default)
./wasmsh/target/release/wasmsh run --dir . ./wasm-tail/target/wasm32-wasi/release/wasm-tail.wasm test.txt

# Run wasm-tail to get the last 3 lines
./wasmsh/target/release/wasmsh run --dir . ./wasm-tail/target/wasm32-wasi/release/wasm-tail.wasm -n 3 test.txt
```