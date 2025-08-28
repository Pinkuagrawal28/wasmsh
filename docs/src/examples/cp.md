# Example: `wasm-cp`

`wasm-cp` is a `cp`-like tool that copies files and directories. It supports recursive copying for directories.

## Building

```sh
cd wasm-cp
cargo build --target wasm32-wasi --release
```

## Usage

### Copying a file

```sh
# Create a test file
_> echo "Hello, copy!" > original.txt

# Copy the file
./wasmsh/target/release/wasmsh run --dir . ./wasm-cp/target/wasm32-wasi/release/wasm-cp.wasm original.txt copy.txt
```

### Copying a directory (recursively)

```sh
# Create a source directory with a file
_> ./wasmsh/target/release/wasmsh run --dir . ./wasm-mkdir/target/wasm32-wasi/release/wasm-mkdir.wasm source_dir
_> echo "Inside source" > source_dir/file_in_source.txt

# Copy the directory
./wasmsh/target/release/wasmsh run --dir . ./wasm-cp/target/wasm32-wasi/release/wasm-cp.wasm source_dir destination_dir
```