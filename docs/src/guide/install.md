# Installing Tools (`install`)

The `install` command downloads a wasm tool from an OCI registry and makes it available in `wasmsh`.

```sh
wasmsh install <image-ref>
```

`image-ref` is a standard OCI image reference, like `registry.com/namespace/tool:tag`.

The installed tools are stored in `~/.wasmsh/bin/`.

## Example

```sh
./target/release/wasmsh install ghcr.io/wasmedge/cowsay:latest
```
This will download `cowsay.wasm` and you can then run it with `wasmsh run` or in the `wasmsh` shell.