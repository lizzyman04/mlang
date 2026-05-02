# Evaluator Internals

Source: `src/core/interpreter/eval.rs`, `src/core/interpreter/stmt.rs`

## Expression evaluation

`eval.rs::evaluate(expr: Expression, env: &mut Environment) -> Result<Expression, String>`

The evaluator is purely recursive — it consumes an `Expression` and returns a reduced `Expression`. Every call must eventually return one of the six literal variants.

### Literal passthrough

`IntLiteral`, `DecLiteral`, `TxtLiteral`, `BoolLiteral`, `ArrayLiteral`, `StructLiteral` evaluate to themselves.

### Identifier lookup

```rust
Identifier(name) => env.get(&name).map(|(_, v)| v.clone())
```

Walks the scope chain. Returns an error if the name is not bound.

### Binary operators

`eval_binary(lv, rv, op)` matches on the triple `(left_value, right_value, operator_kind)`.

Type promotion rules:
- `int op int` → `int`
- `dec op dec` → `dec`
- `int op dec` or `dec op int` → `dec` (lossless widening)
- `txt + anything` → `txt` (right side coerced via `Display`)

Division by zero is a runtime error for both `int` and `dec`.

### Type coercion

`coerce_value(expected: &Type, value: Expression)` is called wherever a value must match a declared type:
- `dec` accepts `int` (widening)
- All other mismatches are hard errors with a descriptive message

`coerce_to_type` in `stmt.rs` is the statement-level variant (includes the variable name in error messages).

### Array methods

`MethodCall` dispatch in `eval.rs`:

| Method | Mutates `env`? | Notes |
|---|---|---|
| `len` | No | Returns element count |
| `contains` | No | Linear scan with `exprs_equal` |
| `slice` | No | Bounds-checked sub-slice |
| `push` | Yes | Appends, returns new length |
| `pop` | Yes | Removes last, returns element |
| `clear` | Yes | Empties, returns 0 |

Mutating methods (`push`, `pop`, `clear`) require the receiver to be a named variable so they can write back to `env`. Calling them on a temporary (e.g., `some_fn().push(x)`) is a runtime error.

### Built-in function dispatch

`FnCall` checks the function name before consulting the user registry:

```
"read"     → print optional prompt, flush stdout, read_line → IntLiteral | DecLiteral | TxtLiteral (auto-detected)
"int"      → cast to int (dec truncates, txt parsed)
"dec"      → cast to dec (int widened, txt parsed)
"txt"      → format as string
_          → look up in env.get_function()
```

### User function calls

1. Look up `FunctionDef` in the environment (error if missing).
2. Validate argument count.
3. Evaluate all arguments.
4. Create a child `Environment` via `env.child_for_call()`.
5. Bind each parameter (with coercion) in the child env.
6. Execute the function body via `execute_stmt`.
7. Return the value from the first `ExecutionResult::Return`, or `IntLiteral(0)` if the body completes without returning.

## Statement execution

`stmt.rs::execute_stmt(stmt: ASTNode, env: &mut Environment, output: Option<&mut &mut String>) -> Result<ExecutionResult, String>`

The `output` parameter is `Some(&mut buf)` when running in test/REPL capture mode and `None` in normal execution. Only `PrintStmt` uses it; all other statements ignore it.

### Control flow propagation

`ExecutionResult` propagates upward:
- `Unit` → continue
- `Return(v)` → bubble up through all statement contexts until consumed by a function body
- `Break` / `Continue` → bubble up until consumed by the nearest loop

Reaching a `Break` or `Continue` outside a loop is a runtime error.
