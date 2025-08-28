# `wasm-chmod`

`wasm-chmod` is a WebAssembly version of the `chmod` command, used to change file permissions.

## Usage

```bash
wasm-chmod <MODE> <FILE>
```

- `<MODE>`: The octal representation of the permissions (e.g., "755").
- `<FILE>`: The path to the file whose permissions are to be changed.

## Examples

Make a file executable by the owner, readable by group and others:

```bash
wasm-chmod 755 my_script.sh
```

Make a file read-only for everyone:

```bash
wasm-chmod 444 my_file.txt
```
