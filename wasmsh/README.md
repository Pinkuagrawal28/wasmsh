# wasmsh

**A tiny shell for running sandboxed WASM command-line tools.**

```
 __      __   _                    _
 \ \    / /  | |                  | |
  \ \  / /__ | |__    __ _   ___  | |__
   \ \/ // _ \| '_ \  / _` | / __| | '_ 
    \  /|  __/| | | || (_| || (__  | | | |
     \/  \___||_| |_| \__,_| \___| |_| |_|
```

`wasmsh` is a prototype of a shell-like environment for running WebAssembly command-line tools. It's like a minimal WSL or Docker for WASM, with a focus on security and sandboxing by default.

## Features

- **Run WASM CLI tools:** Execute `.wasm` files compiled to the WASI standard.
- **Sandboxed by default:** No filesystem or network access unless explicitly granted.
- **OCI Registry Support:** Install tools from OCI-compliant registries (e.g., Docker Hub, GHCR).
- **Interactive Shell:** A `wasmsh>` prompt for a shell-like experience.
- **Simple and Extensible:** A minimal core that can be extended with more features.

## Getting Started

### Installation

1.  **Install Rust:**
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Build `wasmsh`:**
    ```sh
    git clone https://github.com/your-username/wasmsh.git
    cd wasmsh
    cargo build --release
    ```
    The binary will be at `target/release/wasmsh`.

## Usage

`wasmsh` has several commands: `run`, `install`, `list`, and `shell`.

### `install`

Install a tool from an OCI registry.

```sh
wasmsh install <image-ref>
```

Example:
```sh
./target/release/wasmsh install ghcr.io/wasmedge/cowsay:latest
```
This will download the `cowsay.wasm` module and store it in `~/.wasmsh/bin/`.

### `list`

List all installed tools.

```sh
./target/release/wasmsh list
```

### `shell`

Start an interactive `wasmsh>` shell. From here, you can run any installed tool by its name.

```sh
./target/release/wasmsh shell
wasmsh> cowsay "Hello from wasmsh!"
```

### `run`

Run a `.wasm` module directly.

```sh
wasmsh run [OPTIONS] <module> [ARGS]...
```

**Options:**
- `--dir <path>`: Grant read-write access to a directory. Can be used multiple times.
- `--allow-net`: Grant network access.

**Security:**
`wasmsh` will prompt for confirmation before running a module, showing the permissions you are granting.

Example:
```sh
# Assuming you have wasm-grep.wasm (see below)
./target/release/wasmsh run --dir . ./wasm-grep.wasm "hello" file.txt
```

## Example Tool: `wasm-grep`

We have created a simple `grep`-like tool that you can compile to WASM and use with `wasmsh`.

1.  **Build `wasm-grep`:
    ```sh
    # You need the wasm32-wasi target
    rustup target add wasm32-wasi

    cd wasm-grep
    cargo build --target wasm32-wasi --release
    ```
    The wasm module will be at `target/wasm32-wasi/release/wasm-grep.wasm`.

2.  **Use with `wasmsh`:
    Create a file `test.txt` with some content.
    ```
    hello world
    another line
    hello again
    ```
    Now run `wasm-grep` with `wasmsh`:
    ```sh
    # from the root directory of the project
    ./wasmsh/target/release/wasmsh run --dir . ./wasm-grep/target/wasm32-wasi/release/wasm-grep.wasm "hello" test.txt
    ```

## Contributing

This is a prototype and we welcome contributions! Feel free to open issues or pull requests.

## License

This project is licensed under the MIT License.
