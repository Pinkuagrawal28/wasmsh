# `wasm-uniq`

`wasm-uniq` is a WebAssembly version of the `uniq` command, used to report or omit repeated adjacent lines from a file or standard input.

## Usage

```bash
wasm-uniq [FILE]
```

- `[FILE]`: (Optional) The path to the input file. If not provided, `wasm-uniq` reads from standard input.

## Examples

Filter unique lines from a file:

```bash
wasm-uniq my_file.txt
```

Filter unique lines from standard input:

```bash
echo -e "a\na\nb\nc\nc" | wasm-uniq
# Expected output:
# a
# b
# c
```

