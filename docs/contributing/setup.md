# Development Setup

## Prerequisites

- Rust 1.80+ (install via [rustup.rs](https://rustup.rs))
- `cargo` (ships with Rust)
- `git`

## Clone and build

```bash
git clone https://github.com/lizzyman04/mlang.git
cd mlang
cargo build
```

## Run a program

```bash
cargo run -- examples/hello.mth
```

## Run all example tests

There is no automated test harness yet — correctness is verified by running example files and inspecting output:

```bash
cargo run -- examples/test_functions.mth
cargo run -- examples/test_structs.mth
cargo run -- examples/test_array.mth
cargo run -- examples/test_loops.mth
```

## Useful cargo commands

```bash
cargo check          # fast type-check without full compile
cargo build          # debug build
cargo build --release
cargo clippy         # linter
cargo fmt            # auto-format
```

## Editor setup

Any editor with `rust-analyzer` support will give you completion, inline errors, and go-to-definition. Recommended: VS Code with the `rust-analyzer` extension.

## Project layout

See [architecture/overview.md](../architecture/overview.md) for a full module map.
