# Example: `wasm-zip`

`wasm-zip` is a tool for compressing and decompressing files using the DEFLATE/gzip algorithm. It demonstrates how a wasm module can perform computation-heavy tasks.

## Building

```sh
cd wasm-zip
cargo build --target wasm32-wasi --release
```

## Usage

### Compressing a file

```sh
# Create a test file
_> echo "Hello, wasm-zip!" > test.txt

# Compress the file
./wasmsh/target/release/wasmsh run --dir . ./wasm-zip/target/wasm32-wasi/release/wasm-zip.wasm c test.txt test.txt.gz
```

### Decompressing a file

```sh
# Decompress the file
./wasmsh/target/release/wasmsh run --dir . ./wasm-zip/target/wasm32-wasi/release/wasm-zip.wasm d test.txt.gz decompressed.txt

# Check the content
_> cat decompressed.txt
```