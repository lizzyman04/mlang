# Installation

## Install script

```bash
curl -fsSL https://lizzyman04.github.io/mlang/install.sh | sh
```

This downloads the latest release binary and places it in `/usr/local/bin/mlang` (Linux/macOS) or `~/bin/mlang` (Windows).

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

No dedicated extension exists yet. For basic syntax highlighting, configure your editor to treat `.mth` files as a generic scripting language with C-style block structure.
