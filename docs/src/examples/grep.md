# Example: `wasm-grep`

`wasm-grep` is a simple, `grep`-like tool that searches for a pattern in a file. It's a good example of a wasm module that interacts with the filesystem.

## Building

```sh
cd wasm-grep
cargo build --target wasm32-wasi --release
```

## Usage

To run `wasm-grep`, you need to grant it access to the directory containing the file you want to search.

```sh
# Create a test file
_> echo "hello world\nhello wasm" > test.txt

# Run wasm-grep with wasmsh
./wasmsh/target/release/wasmsh run --dir . ./wasm-grep/target/wasm32-wasi/release/wasm-grep.wasm "hello" test.txt
```

This will output:
```
hello world
hello wasm
```