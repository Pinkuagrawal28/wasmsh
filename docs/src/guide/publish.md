# Publishing Tools (`publish`)

The `publish` command allows you to publish your own wasm tools to an OCI registry.

```sh
wasmsh publish <module> <image-ref>
```

- `<module>`: The path to the `.wasm` file you want to publish.
- `<image-ref>`: The OCI image reference to publish to, e.g., `your-registry.com/your-name/your-tool:latest`.

## Authentication

To publish to a registry, you need to be authenticated. `wasmsh` will use the credentials from your Docker config file (`~/.docker/config.json`). You can authenticate by running `docker login your-registry.com` before using the `publish` command.

## Example

```sh
./target/release/wasmsh publish ./wasm-grep/target/wasm32-wasi/release/wasm-grep.wasm your-registry.com/your-name/wasm-grep:0.1.0
```