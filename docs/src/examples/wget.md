# `wasm-wget`

`wasm-wget` is a WebAssembly version of the `wget` command, used to download files from the web.

## Usage

```bash
wasm-wget <URL> [--output <FILE>]
```

- `<URL>`: The URL of the file to download.
- `--output <FILE>`: (Optional) The name of the file to save the downloaded content to. If not provided, the content is printed to standard output.

## Examples

Download a file and save it:

```bash
wasm-wget https://example.com/file.txt --output file.txt
```

Download a file and print to stdout:

```bash
wasm-wget https://example.com/data.json
```
