# `wasm-gzip`

`wasm-gzip` is a WebAssembly version of the `gzip` and `gunzip` commands, used for compressing and decompressing files.

## Usage

### Compress

```bash
wasm-gzip <INPUT_FILE> [--output <OUTPUT_FILE>]
```

- `<INPUT_FILE>`: The file to compress.
- `--output <OUTPUT_FILE>`: (Optional) The name of the compressed output file. If not provided, the output file will be named `<INPUT_FILE>.gz`.

### Decompress

```bash
wasm-gzip --decompress <INPUT_FILE> [--output <OUTPUT_FILE>]
```

- `--decompress`: Flag to indicate decompression.
- `<INPUT_FILE>`: The file to decompress (expected to have a `.gz` extension).
- `--output <OUTPUT_FILE>`: (Optional) The name of the decompressed output file. If not provided, the output file will be named by removing the `.gz` extension from the input file name.

## Examples

Compress a file:

```bash
wasm-gzip my_file.txt
# Creates my_file.txt.gz
```

Decompress a file:

```bash
wasm-gzip --decompress my_file.txt.gz
# Creates my_file.txt
```

Compress with a specified output name:

```bash
wasm-gzip my_file.txt --output compressed.gz
```
