#!/bin/bash

set -euo pipefail

TOOL_NAME="$1"

if [[ -z "$TOOL_NAME" ]]; then
    echo "Usage: $0 <tool-name>"
    exit 1
fi

PROJECT_DIR="wasm-$TOOL_NAME"
FULL_PATH="$(pwd)/$PROJECT_DIR"
SRC_PATH="$FULL_PATH/src"
CARGO_TOML="$FULL_PATH/Cargo.toml"
MAIN_RS="$SRC_PATH/main.rs"
DOCS_SUMMARY="$(pwd)/docs/src/SUMMARY.md"
DOCS_EXAMPLE="$(pwd)/docs/src/examples/$TOOL_NAME.md"

# 1. Create directory structure
echo "Creating directory: $FULL_PATH/src"
mkdir -p "$SRC_PATH"

# 2. Create Cargo.toml
echo "Creating Cargo.toml: $CARGO_TOML"
cat << EOF > "$CARGO_TOML"
[package]
name = "$PROJECT_DIR"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
clap = { version = "4.0", features = ["derive"] }
EOF

# 3. Create src/main.rs
echo "Creating src/main.rs: $MAIN_RS"
cat << EOF > "$MAIN_RS"
use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() -> Result<()> {
    println!("Hello from wasm-$TOOL_NAME!");
    Ok(())
}
EOF

# 4. Add to docs/src/SUMMARY.md (alphabetically)
echo "Updating docs/src/SUMMARY.md"
# Find the line to insert after (e.g., after the last wasm- entry alphabetically before the new one)
# This is a simplified insertion. For robust alphabetical insertion, a more complex script or tool would be needed.
# For now, we'll just append to the Examples section and rely on manual sorting if needed.

# Get the last line of the examples section
LAST_EXAMPLE_LINE=$(grep -n "- \`wasm-" "$DOCS_SUMMARY" | tail -1 | cut -d: -f1)

if [[ -n "$LAST_EXAMPLE_LINE" ]]; then
    # Insert after the last example line
    sed -i "${LAST_EXAMPLE_LINE}a\
- \`$PROJECT_DIR\`\(examples/$TOOL_NAME.md\)" "$DOCS_SUMMARY"
else
    # If no examples exist, append to the Examples section header
    sed -i "/\# Examples/a\
\
- \`$PROJECT_DIR\`\(examples/$TOOL_NAME.md\)" "$DOCS_SUMMARY"
fi

# 5. Create docs/src/examples/<tool_name>.md
echo "Creating docs/src/examples/$TOOL_NAME.md"
cat << EOF > "$DOCS_EXAMPLE"
# \`$PROJECT_DIR\`

\`$PROJECT_DIR\` is a WebAssembly version of the \`$TOOL_NAME\` command.

## Usage

```bash
$PROJECT_DIR
```

## Examples

Basic usage:

```bash
$PROJECT_DIR
```
EOF

echo "Successfully created $PROJECT_DIR and updated documentation."
