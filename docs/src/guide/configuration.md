# Configuration

`wasmsh` can be configured with a TOML file located at `~/.wasmsh/config.toml`.

If the file does not exist, `wasmsh` will create it with some default comments.

## Aliases

You can define aliases for commands in the `[aliases]` table. This is useful for creating shortcuts for your favorite tools.

```toml
# ~/.wasmsh/config.toml

[aliases]
g = "wasm-grep"
wc = "wasm-curl"
```

With this configuration, you can run `g` in the `wasmsh` shell as a shortcut for `wasm-grep`.