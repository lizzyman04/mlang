# Built-in Functions

MLang has two categories of built-ins: **statement forms** (parsed at the syntax level) and **function call forms** (intercepted in the evaluator before user-function lookup).

## Statement forms

### `print(expr)`

Parsed as `ASTNode::PrintStmt` in `stmt/print.rs`. Evaluated in `stmt.rs`:

1. Evaluate the inner expression to a literal.
2. Format via `format_value()` — handles all six literal types, including nested arrays and struct instances.
3. Write to `output` buffer (test mode) or `println!` to stdout.

`print` is not an `FnCall` — it cannot be used as an expression or passed as a value.

## Function call forms (intercepted in `eval.rs`)

### `read([prompt: txt]) → txt`

1. If an argument is provided, evaluate it; assert it is `TxtLiteral`.
2. `print!` the prompt (no newline); flush stdout.
3. `stdin().read_line()` into a buffer.
4. `trim()` the result and infer the return type:
   - empty → `TxtLiteral`
   - parses as `i64` → `IntLiteral`
   - parses as `f64` → `DecLiteral`
   - otherwise → `TxtLiteral`

### `int(x) → int`

| Input type | Output |
|---|---|
| `int` | unchanged |
| `dec` | truncated with `as i64` |
| `txt` | parsed via `str::parse::<i64>()` |
| other | runtime error |

### `dec(x) → dec`

| Input type | Output |
|---|---|
| `dec` | unchanged |
| `int` | widened with `as f64` |
| `txt` | parsed via `str::parse::<f64>()` |
| other | runtime error |

### `txt(x) → txt`

| Input type | Output |
|---|---|
| `txt` | unchanged |
| `int` | `i64::to_string()` |
| `dec` | `f64::to_string()` |
| `bool` | `"true"` or `"false"` |
| other | runtime error |

## Array methods (dispatched in `eval.rs::MethodCall`)

These are not `FnCall`s — they are method call expressions on an array receiver.

| Method | Signature | Notes |
|---|---|---|
| `.len()` | `→ int` | Count of elements |
| `.contains(v)` | `→ bool` | Linear scan, compares literals by value |
| `.slice(s, e)` | `→ array<T>` | Exclusive end; bounds-checked |
| `.push(v)` | `→ int` | Appends `v`, returns new length; mutates in-place |
| `.pop()` | `→ T` | Removes and returns last element; mutates in-place |
| `.clear()` | `→ int` | Empties the array, returns 0; mutates in-place |

Mutating methods rewrite the variable in `env` after modifying the element list. The receiver must be a named variable (`Identifier`), not an arbitrary expression.
