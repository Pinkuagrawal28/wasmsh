# Introduction

`wasmsh` is a prototype of a shell-like environment for running WebAssembly command-line tools. It's like a minimal WSL or Docker for WASM, with a focus on security and sandboxing by default.

## Features

- **Run WASM CLI tools:** Execute `.wasm` files compiled to the WASI standard.
- **Sandboxed by default:** No filesystem or network access unless explicitly granted.
- **OCI Registry Support:** Install and publish tools from OCI-compliant registries (e.g., Docker Hub, GHCR).
- **Interactive Shell:** A `wasmsh>` prompt for a shell-like experience with history and aliases.
- **Configurable:** Use a simple TOML file to configure aliases.

## Built-in WASM Tools

`wasmsh` aims to provide a familiar command-line experience by offering a suite of common Unix-like utilities, re-implemented in WebAssembly. These tools leverage the sandboxed nature of WASM to provide secure and portable functionalities.

Currently, the following categories of tools have been built:

- **File System Utilities:** `wasm-chmod`, `wasm-cp`, `wasm-df`, `wasm-du`, `wasm-find`, `wasm-ls`, `wasm-mkdir`, `wasm-mv`, `wasm-pwd`, `wasm-rm`, `wasm-rmdir`, `wasm-touch`
- **Text Processing:** `wasm-cat`, `wasm-echo`, `wasm-grep`, `wasm-sort`, `wasm-uniq`, `wasm-wc`
- **Networking:** `wasm-curl`, `wasm-wget`
- **System Information:** `wasm-date`, `wasm-hostname`, `wasm-uname`, `wasm-whoami`
- **Archiving:** `wasm-gzip`, `wasm-tar`, `wasm-zip`

Each tool is designed to mimic its traditional Unix counterpart as closely as possible within the constraints of the WebAssembly environment. For detailed usage and examples of each tool, please refer to the [Examples](examples/grep.md) section in the documentation.
