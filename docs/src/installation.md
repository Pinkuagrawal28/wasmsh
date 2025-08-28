# Installation

To get started with `wasmsh`, you'll need to have the Rust toolchain installed.

1.  **Install Rust:**
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Build `wasmsh`:**
    Clone the repository and build the project:
    ```sh
    git clone https://github.com/your-username/wasmsh.git
    cd wasmsh
    cargo build --release
    ```
    The `wasmsh` binary will be available at `target/release/wasmsh`.

3.  **Build the example tools:**
    `wasmsh` comes with two example tools, `wasm-grep` and `wasm-curl`. To build them, you'll need the `wasm32-wasi` target:
    ```sh
    rustup target add wasm32-wasi
    ```
    Then, you can build the tools:
    ```sh
    cd wasm-grep
    cargo build --target wasm32-wasi --release
    cd ../wasm-curl
    cargo build --target wasm32-wasi --release
    ```