# Running Tools (`run`)

The `run` command is used to execute a `.wasm` module directly.

```sh
wasmsh run [OPTIONS] <module> [ARGS]...
```

## Options

- `--dir <path>`: Grant read-write access to a directory on the host. The directory will be available in the guest at a path matching the directory name. This flag can be used multiple times.
- `--allow-net`: Grant network access to the wasm module.

## Security Prompt

For security, `wasmsh` will always print a summary of the permissions being granted and ask for your confirmation before executing the module.

```
Running module: ./wasm-curl/target/wasm32-wasi/release/wasm-curl.wasm
Permissions:
  - Filesystem: (none)
  - Network: all
Proceed? [y/N]
```

## Example

```sh
./wasmsh/target/release/wasmsh run --dir . ./wasm-grep/target/wasm32-wasi/release/wasm-grep.wasm "hello" test.txt
```