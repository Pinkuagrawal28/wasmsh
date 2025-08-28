# Example: `wasm-cat`

`wasm-cat` is a `cat`-like tool that prints the content of one or more files to the console.

## Building

```sh
cd wasm-cat
cargo build --target wasm32-wasi --release
```

## Usage

To run `wasm-cat`, you need to grant it access to the directory containing the files you want to display.

```sh
# Create a test file
_> echo "Hello from wasm-cat" > test.txt

# Run wasm-cat with wasmsh
./wasmsh/target/release/wasmsh run --dir . ./wasm-cat/target/wasm32-wasi/release/wasm-cat.wasm test.txt
```