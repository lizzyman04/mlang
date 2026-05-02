# Installation

## Build from source (recommended)

```bash
git clone https://github.com/lizzyman04/mlang.git
cd mlang
cargo build --release
```

The compiled binary is at `target/release/mlang`. Copy it anywhere on your `PATH`:

```bash
# Linux / macOS
sudo cp target/release/mlang /usr/local/bin/mlang

# or add the release dir to PATH in your shell profile
export PATH="$HOME/mlang/target/release:$PATH"
```

Verify:

```bash
mlang --version
```

## Running programs

```bash
mlang path/to/program.mth
```

## File extension

MLang source files use the `.mth` extension.

## Editor support

No dedicated extension exists yet. For syntax highlighting, configure your editor to treat `.mth` files as a Rust-like language — the block structure and keywords are similar enough for basic coloring.

## Updating

```bash
git pull
cargo build --release
```
