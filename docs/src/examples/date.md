# Example: `wasm-date`

`wasm-date` is a `date`-like tool that prints the current UTC date and time. It demonstrates how a wasm module can access the system clock.

## Building

```sh
cd wasm-date
cargo build --target wasm32-wasi --release
```

## Usage

The `wasm-date` tool does not require any special permissions to run.

```sh
./wasmsh/target/release/wasmsh run ./wasm-date/target/wasm32-wasi/release/wasm-date.wasm
```

This will output the current date and time in UTC, for example:
```
2023-10-27 10:30:00 UTC
```