# Math Engine Overview

MLang's math engine is an integrated symbolic and numeric computation system embedded directly into the language runtime. No imports, no external libraries — algebraic manipulation is first-class.

---

## Purpose

Traditional languages treat math as a library concern. MLang treats it as a language feature. The engine handles:

- Solving linear and polynomial equations symbolically
- Simplifying and evaluating expressions
- Tracking every transformation step for introspection
- Exposing results as structured objects with a full method API

---

## Entry Points

Three top-level functions are the primary interface to the engine:

| Function | Input | Output | Description |
|---|---|---|---|
| `math.solve(eq)` | equation string | `MathExpr` | Solve for variable, return introspectable object |
| `math.simplify(expr)` | expression string | `txt` | Reduce expression to simplest form |
| `math.evaluate(expr)` | expression string | `dec` | Numerically evaluate to a decimal |

### Examples

```mlang
expr = math.solve("2x + 4 = 10");
print(expr.result());   // x = 3

simplified = math.simplify("2x + x + 4 - 1");
print(simplified);      // 3x + 3

val = math.evaluate("3 * pi + 1");
print(val);             // 10.42477796076938
```

---

## Design Principles

**Lazy evaluation** — `math.solve()` does not compute until `.result()` or a step method is called. This allows the object to be passed, stored, and inspected without premature computation.

**Step storage** — every transformation is recorded as a string snapshot. Steps are 0-indexed. Step 0 is always the original equation.

**Symbolic-first** — the engine prefers symbolic results (`x = 3`, `r = sqrt(100/pi)`) over float approximations. Call `.evaluate()` explicitly to get a decimal.

---

## Constants Available in Expressions

All constants are accessible directly inside equation strings and as `math.<name>` values in MLang code.

| Constant | Value |
|---|---|
| `pi` | 3.141592653589793 |
| `e` | 2.718281828459045 |
| `tau` | 6.283185307179586 |
| `inf` | ∞ |

```mlang
area = math.evaluate("pi * 5^2");
print(area);    // 78.53981633974483

print(math.pi); // 3.141592653589793
```

---

## See Also

- [`api.md`](api.md) — full `MathExpr` object method reference
- [`functions.md`](functions.md) — numeric utility functions (`sqrt`, `sin`, `log`, etc.)
- [`constants.md`](constants.md) — constant values and usage
- [`architecture.md`](architecture.md) — internal pipeline (lexer → parser → transformer → solver)
