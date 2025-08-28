# Example: `wasm-ln`

`wasm-ln` is an `ln`-like tool that creates links (shortcuts) to files. It supports both hard links and symbolic links.

## Building

```sh
cd wasm-ln
cargo build --target wasm32-wasi --release
```

## Usage

### Creating a hard link

```sh
# Create a test file
_> echo "Hello, link!" > original.txt

# Create a hard link
./wasmsh/target/release/wasmsh run --dir . ./wasm-ln/target/wasm32-wasi/release/wasm-ln.wasm original.txt hardlink.txt
```

### Creating a symbolic link

```sh
# Create a symbolic link
./wasmsh/target/release/wasmsh run --dir . ./wasm-ln/target/wasm32-wasi/release/wasm-ln.wasm -s original.txt symlink.txt
```
