# Example: `wasm-rm`

`wasm-rm` is an `rm`-like tool that removes files and directories. It supports recursive removal for directories.

## Building

```sh
cd wasm-rm
cargo build --target wasm32-wasi --release
```

## Usage

### Removing a file

```sh
# Create a test file
_> echo "Delete me!" > delete_me.txt

# Remove the file
./wasmsh/target/release/wasmsh run --dir . ./wasm-rm/target/wasm32-wasi/release/wasm-rm.wasm delete_me.txt
```

### Removing a directory (recursively)

```sh
# Create a directory with a file inside
_> ./wasmsh/target/release/wasmsh run --dir . ./wasm-mkdir/target/wasm32-wasi/release/wasm-mkdir.wasm dir_to_delete
_> echo "Inside dir" > dir_to_delete/file_inside.txt

# Remove the directory
./wasmsh/target/release/wasmsh run --dir . ./wasm-rm/target/wasm32-wasi/release/wasm-rm.wasm dir_to_delete
```
