# WASM Tools Project

This repository contains a collection of command-line tools compiled to WebAssembly (WASM) and a prototype shell (`wasmsh`) designed to run them in a sandboxed environment. The goal is to explore the capabilities of WebAssembly for building secure, portable, and efficient command-line utilities.

## Project Structure

*   **`wasmsh/`**: The core WebAssembly shell that provides a sandboxed environment for running WASM CLI tools.
*   **`wasm-*/`**: Individual command-line utilities (e.g., `wasm-cat`, `wasm-ls`, `wasm-grep`) implemented in Rust and compiled to WASI (WebAssembly System Interface).
*   **`docs/`**: Project documentation, including guides and examples.

## Features

*   **Run WASM CLI tools:** Execute `.wasm` files compiled to the WASI standard.
*   **Sandboxed by default:** No filesystem or network access unless explicitly granted, ensuring security.
*   **OCI Registry Support:** Install tools from OCI-compliant registries (e.g., Docker Hub, GHCR) via `wasmsh`.
*   **Interactive Shell:** A `wasmsh>` prompt for a shell-like experience.
*   **Simple and Extensible:** A minimal core that can be extended with more features.

## Getting Started

To get started with `wasmsh` and the WASM tools, you'll need Rust and `wasm32-wasi` target.

1.  **Install Rust:**
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Add WASI target:**
    ```sh
    rustup target add wasm32-wasi
    ```

3.  **Build `wasmsh`:**
    ```sh
    git clone https://github.com/Pinkuagrawal28/wasmsh.git .
    cargo build --release --package wasmsh
    ```
    The `wasmsh` binary will be at `target/release/wasmsh`.

4.  **Build individual WASM tools (example: `wasm-grep`):**
    ```sh
    cd wasm-grep
    cargo build --target wasm32-wasi --release
    ```
    The WASM module will be at `target/wasm32-wasi/release/wasm-grep.wasm`.

## Usage

Refer to the `wasmsh/README.md` and the `docs/` directory for detailed usage instructions for `wasmsh` and the individual tools.

## Contributing

This project is a prototype, and contributions are welcome! Feel free to open issues or pull requests.

## License

This project is licensed under the MIT License.
