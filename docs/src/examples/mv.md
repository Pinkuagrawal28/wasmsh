# Example: `wasm-mv`

`wasm-mv` is a `mv`-like tool that moves or renames files and directories.

## Building

```sh
cd wasm-mv
cargo build --target wasm32-wasi --release
```

## Usage

```sh
# Create a test file
_> echo "Hello, move!" > original.txt

# Move/rename the file
./wasmsh/target/release/wasmsh run --dir . ./wasm-mv/target/wasm32-wasi/release/wasm-mv.wasm original.txt renamed.txt
```
