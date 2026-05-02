# Testing

## Current approach

MLang uses example `.mth` files as integration tests. There are no unit tests — the interpreter is tested end-to-end by running programs and observing output.

## Running example files

```bash
cargo run -- examples/hello.mth
cargo run -- examples/test_functions.mth
cargo run -- examples/test_structs.mth
cargo run -- examples/test_array.mth
cargo run -- examples/test_loops.mth
cargo run -- examples/test_io.mth     # interactive, requires stdin
```

## Writing a test file

1. Create `examples/test_<feature>.mth`.
2. Cover the happy path and at least one edge case (empty array, zero value, nested struct, etc.).
3. Use `print()` to assert expected values — check output visually or via shell comparison:

```bash
cargo run -- examples/test_functions.mth | diff - expected_output.txt
```

## Error cases

To verify that the interpreter correctly rejects invalid input, test by running a known-bad program and checking that it exits with an error:

```bash
cargo run -- /tmp/bad.mth 2>&1 | grep "Type mismatch"
```

## Adding Rust unit tests (future)

The `execute()` function in `executor.rs` accepts `output: Option<&mut String>` for exactly this purpose — captured output can be compared in a Rust `#[test]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{lexer::mod::tokenize, parser::parse::entry::parse};

    fn run(src: &str) -> String {
        let tokens = tokenize(src).unwrap();
        let ast    = parse(tokens).unwrap();
        let mut out = String::new();
        execute(ast, Some(&mut out)).unwrap();
        out
    }

    #[test]
    fn test_add() {
        assert_eq!(run("int add(int a, int b){return a+b;} main(){print(add(2,3));}"), "5\n");
    }
}
```

This pattern is ready to use — no test infrastructure changes are needed.
