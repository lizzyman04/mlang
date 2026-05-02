# Conventions

## Commit messages

Single-line, imperative mood, no trailing period. Use a scope prefix:

```
feat(lexer): add struct keyword
feat(parser): implement struct parsing
feat(eval): implement struct instantiation and field access
test: add struct test suite
fix: handle empty array in slice bounds check
refactor(eval): extract eval_binary into helper
docs: add architecture overview
```

No bodies, no `Co-Authored-By`, no `#issue` references in the message line.

## Branch names

```
feat/structs
fix/array-bounds
docs/landing-page
```

## Code style

- `cargo fmt` before committing — no manual formatting debates.
- `cargo clippy` should produce no warnings on new code.
- Follow the existing module structure — one concern per file.
- No `unwrap()` in library code; use `?` or explicit error messages.
- No `println!` in library code; output goes through `execute_stmt`'s `output` parameter.

## File naming

- Rust source: `snake_case.rs`
- MLang source: `snake_case.mth`
- Docs: `kebab-case.md`

## Error messages

Runtime errors should name the construct, the expected type/value, and what was found:

```
Type mismatch: variable 'age' declared as 'int' but got 'txt'.
Expected '(' after 'int' in cast expression
Undefined variable 'name'
Index 5 out of bounds for array of length 3
```

Avoid vague messages like "invalid expression" without context.

## Adding a language feature

See [adding-a-feature.md](./adding-a-feature.md) for the end-to-end workflow.
