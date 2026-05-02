# Adding a Language Feature

This guide walks through adding a new language feature end-to-end using the existing codebase as a template. As an example, imagine adding a `not_eq` infix operator `<>`.

## The five-layer checklist

Every new language feature touches up to five layers. Work top-down.

---

### Layer 1 — Lexer (`src/core/lexer/`)

If the feature requires a new token, add it here.

1. **Add the `TokenKind` variant** in `token.rs` (or extend an existing symbol enum in `symbol/`).
2. **Teach the tokenizer** to produce it in `tokenizer/symbols.rs` (or `identifiers.rs` for keywords).
3. **Register it as a keyword** in `rules.rs::is_keyword()` if it's a reserved word.

Example: adding `<>` as `ComparisonSymbol(NotEqual2)` would extend `ComparisonSymbolKind` and the multi-char symbol dispatch in `symbols.rs`.

---

### Layer 2 — Parser (`src/core/parser/`)

If the feature needs new syntax:

1. **Add a new `ASTNode` variant** in `ast/node.rs` (statements/declarations) or a new `Expression` variant in `ast/expr.rs` (value-producing constructs).
2. **Write a parser function** under `stmt/` or extend `parse/expr.rs`.
3. **Hook it into `parse_statement`** (`stmt/parser.rs`) or the appropriate precedence level in `parse/expr.rs`.

For an expression operator: add the token check at the correct precedence level (e.g., `parse_comparison` for `<>`).

---

### Layer 3 — Evaluator (`src/core/interpreter/eval.rs` or `stmt.rs`)

Handle the new AST node or expression variant:

- **Expressions** → add a match arm in `evaluate()`.
- **Statements** → add a match arm in `execute_stmt()`.
- **Built-in functions** → add a name check at the top of the `FnCall` arm in `evaluate()`.

For `<>`: add `(lv, rv, ComparisonSymbol(NotEqual2)) => Ok(BoolLiteral(lv != rv))` in `eval_binary`.

---

### Layer 4 — Type system (if needed)

- New types → add a variant to `Type` in `ast/types.rs`.
- Update `coerce_value` / `coerce_to_type` for the new type.
- Update `infer_type` in `rules.rs`.
- Update `format_value` in `stmt.rs` for print support.

---

### Layer 5 — Example file

Add or extend a file in `examples/` that exercises every new behaviour. Name it `test_<feature>.mth`. This serves as the integration test.

---

## Concrete walkthrough: adding `string.upper()` method

1. **Lexer**: no change — `upper` is an identifier, parsed as a method name.
2. **Parser**: no change — `MethodCall` already captures arbitrary method names.
3. **Evaluator**: in `eval.rs`, add to the `MethodCall` dispatch:
   ```rust
   "upper" => match obj_val {
       TxtLiteral(s) => Ok(TxtLiteral(s.to_uppercase())),
       _ => Err("'upper' requires a txt value".to_string()),
   },
   ```
4. **Example**:
   ```mlang
   main() {
       txt s = "hello";
       print(s.upper());   # HELLO
   }
   ```

That's it — no type system changes needed.

---

## Commit convention

Use the prefix that matches the layer touched:

| Prefix | Layer |
|---|---|
| `feat(lexer)` | Lexer |
| `feat(parser)` | Parser / AST |
| `feat(eval)` | Evaluator |
| `feat(builtins)` | Built-in functions |
| `test` | Example test files |
| `fix` | Bug fix |
| `refactor` | Internal refactor |
| `docs` | Documentation |
