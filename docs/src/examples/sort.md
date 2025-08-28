# `wasm-sort`

`wasm-sort` is a WebAssembly version of the `sort` command, used to sort lines of text files.

## Usage

```bash
wasm-sort [FILE]
```

- `[FILE]`: (Optional) The path to the input file. If not provided, `wasm-sort` reads from standard input.

## Examples

Sort lines from a file:

```bash
wasm-sort my_file.txt
```

Sort lines from standard input:

```bash
echo -e "b\na\nc" | wasm-sort
```

