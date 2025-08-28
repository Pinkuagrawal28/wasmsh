# `wasm-find`

`wasm-find` is a WebAssembly version of the `find` command, used to search for files and directories within a specified path.

## Usage

```bash
wasm-find <path> [--name <pattern>]
```

- `<path>`: The starting directory for the search.
- `--name <pattern>`: (Optional) A pattern to match against file or directory names. Currently, this performs a simple substring match.

## Examples

Search for all files and directories in the current directory and its subdirectories:

```bash
wasm-find .
```

Search for files containing "test" in their name within the current directory:

```bash
wasm-find . --name test
```
