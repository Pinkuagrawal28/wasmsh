# Example: `wasm-tar`

`wasm-tar` is a tool for creating and extracting `.tar` archives. It's our most complex tool yet and demonstrates advanced file I/O.

## Building

```sh
cd wasm-tar
cargo build --target wasm32-wasi --release
```

## Usage

### Creating an archive

```sh
# Create some test files
_> echo "file1" > file1.txt
_> echo "file2" > file2.txt

# Create the archive
./wasmsh/target/release/wasmsh run --dir . ./wasm-tar/target/wasm32-wasi/release/wasm-tar.wasm c archive.tar file1.txt file2.txt
```

### Extracting an archive

```sh
# Make a directory to extract into
_> mkdir extracted
_> cd extracted

# Extract the archive
../wasmsh/target/release/wasmsh run --dir . ../wasm-tar/target/wasm32-wasi/release/wasm-tar.wasm x ../archive.tar
```