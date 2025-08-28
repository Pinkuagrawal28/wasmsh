# Example: `wasm-curl`

`wasm-curl` is a simple, `curl`-like tool that fetches the content of a URL. It's a good example of a wasm module that uses networking.

## Building

```sh
cd wasm-curl
cargo build --target wasm32-wasi --release
```

## Usage

To run `wasm-curl`, you need to grant it network access with the `--allow-net` flag.

```sh
./wasmsh/target/release/wasmsh run --allow-net ./wasm-curl/target/wasm32-wasi/release/wasm-curl.wasm https://example.com
```

This will fetch the HTML content of `example.com` and print it to the console.