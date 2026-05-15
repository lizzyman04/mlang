# Math Engine Architecture

The math engine is an internal pipeline that processes equation strings into structured results. It runs entirely within the MLang interpreter — no external CAS (Computer Algebra System) is required.

---

## Pipeline Overview

```
math.solve("2x + 4 = 10")
        │
        ▼
   ┌─────────┐
   │  Lexer  │  tokenize the equation string
   └────┬────┘
        │  [2, x, +, 4, =, 10]
        ▼
   ┌─────────┐
   │ Parser  │  build AST from token stream
   └────┬────┘
        │       =
        │      / \
        │    +   10
        │   / \
        │  2x   4
        ▼
   ┌────────────┐
   │ Normalizer │  rewrite to canonical form
   └─────┬──────┘
         │  LHS - RHS = 0  →  2x - 6 = 0
         ▼
   ┌─────────────┐
   │ Transformer │  isolate variable, collect terms
   └──────┬──────┘
          │  step: "2x = 6"
          ▼
   ┌────────┐
   │ Solver │  symbolic or numeric solution
   └────┬───┘
        │  step: "x = 3"
        ▼
   ┌──────────┐
   │ Renderer │  format steps as strings
   └────┬─────┘
        │
        ▼
   MathExpr { steps: ["2x + 4 = 10", "2x = 6", "x = 3"] }
```

---

## Stage 1: Lexer

Tokenizes the equation string into typed tokens.

**Input:** `"2x + 4 = 10"`  
**Output:** `[Num(2), Ident("x"), Plus, Num(4), Eq, Num(10)]`

Token types:
- `Num(f64)` — numeric literal
- `Ident(String)` — variable or constant name
- `Plus`, `Minus`, `Star`, `Slash`, `Caret` — operators
- `Eq` — equality sign
- `LParen`, `RParen` — grouping
- `Func(String)` — known function name (`sqrt`, `sin`, `log`, etc.)

Constants (`pi`, `e`, `tau`, `inf`) are substituted with their values during lexing.

---

## Stage 2: Parser

Converts the flat token stream into an Abstract Syntax Tree (AST) using recursive descent parsing. Operator precedence is handled at this stage.

**Precedence (low to high):**

1. `=` (equality)
2. `+`, `-` (additive)
3. `*`, `/` (multiplicative)
4. `-` (unary negation)
5. `^` (exponentiation, right-associative)
6. Function calls, parentheses, atoms

**AST node types:**

| Node | Fields |
|---|---|
| `BinOp` | `op`, `left`, `right` |
| `UnaryMinus` | `operand` |
| `Num` | `value: f64` |
| `Var` | `name: String` |
| `FuncCall` | `name`, `arg` |
| `Equation` | `lhs`, `rhs` |

---

## Stage 3: Normalizer

Rewrites the AST into a canonical form before solving. Normalization:

1. Moves all terms to the LHS: `LHS - RHS = 0`
2. Expands multiplication over addition
3. Combines like terms (numeric coefficients on the same variable)
4. Sorts terms by degree (highest power first)

**Example:**

```
2y + 5 = 3y - 1
→ 2y + 5 - 3y + 1 = 0
→ -y + 6 = 0
```

---

## Stage 4: Transformer

Applies algebraic rules step-by-step to isolate the target variable. Each transformation is recorded as a string snapshot and appended to the step list.

**Linear strategy:**

1. Identify the variable to solve for (the only variable in the expression)
2. Separate variable terms from constant terms
3. Divide both sides by the variable's coefficient

**Step recording example:**

```
"3x + 6 = 12"   ← step 0 (original)
"3x = 6"        ← step 1 (subtract 6 from both sides)
"x = 2"         ← step 2 (divide by 3)
```

---

## Stage 5: Solver

Handles cases the transformer cannot resolve algebraically through symbolic rules:

- Polynomial roots (quadratic formula for degree 2)
- Symbolic preservation (`sqrt`, `log`, `e^x` in results)
- Detection of no-solution and infinite-solution cases

For numeric results, the solver evaluates the symbolic answer to a `f64` and formats it.

---

## Stage 6: Renderer

Formats the internal AST nodes and intermediate states into human-readable strings. All steps stored in `MathExpr.steps` are produced by the renderer.

**Formatting rules:**
- `1 * x` → `x`
- `-1 * x` → `-x`
- `x^1` → `x`
- `x^0` → `1`
- `x + -3` → `x - 3`
- Symbolic results preserve function names: `sqrt(100 / pi)`

---

## Implementation Location

| Component | Path |
|---|---|
| Math engine entry | `src/builtins/math/mod.rs` |
| Solver | `src/builtins/math/solver/mod.rs` |
| Interpreter integration | `src/core/interpreter/eval.rs` |

The engine is invoked via the builtin dispatch in the interpreter's `eval` stage. `math.solve`, `math.simplify`, `math.evaluate`, and all `math.*` functions and constants are registered there.

---

## Memory Model

- Each `math.solve()` call allocates a `MathExpr` on the heap
- Steps are stored as a `Vec<String>` — one allocation per transformation
- The original equation string is cloned and stored independently of the step list
- `MathExpr` objects are garbage-collected when they go out of scope (MLang uses reference counting)

---

## See Also

- [`api.md`](api.md) — `MathExpr` method reference
- [`functions.md`](functions.md) — numeric utility functions
- [`constants.md`](constants.md) — built-in constants
- [`../internals/evaluator.md`](../internals/evaluator.md) — how builtins integrate with the interpreter
