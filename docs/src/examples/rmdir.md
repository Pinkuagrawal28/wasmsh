# Example: `wasm-rmdir`

`wasm-rmdir` is an `rmdir`-like tool that removes empty directories.

## Building

```sh
cd wasm-rmdir
cargo build --target wasm32-wasi --release
```

## Usage

```sh
# Create an empty directory
./wasmsh/target/release/wasmsh run --dir . ./wasm-mkdir/target/wasm32-wasi/release/wasm-mkdir.wasm empty_dir

# Remove the empty directory
./wasmsh/target/release/wasmsh run --dir . ./wasm-rmdir/target/wasm32-wasi/release/wasm-rmdir.wasm empty_dir
```