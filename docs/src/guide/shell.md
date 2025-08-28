# Interactive Shell (`shell`)

The `shell` command starts an interactive `wasmsh>` prompt.

```sh
./target/release/wasmsh shell
```

From within the shell, you can run any installed tool by its name, without having to type `wasmsh run`.

## Features

- **Command History:** Use the up and down arrow keys to navigate through your command history. The history is saved in `~/.wasmsh/history.txt`.
- **Aliases:** You can define aliases for your commands in the `~/.wasmsh/config.toml` file.

## Example

```
wasmsh> list
Installed tools:
- cowsay
wasmsh> cowsay "Hello from the wasmsh shell!"
 _____________ 
< Hello from the wasmsh shell! >
 ------------- 
        \   ^__^
         \  (oo)\_______
            (__)\       )/\/
                ||----w |
                ||     ||

wasmsh> exit
```