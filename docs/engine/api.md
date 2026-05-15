# MathExpr API Reference

`math.solve()`, `math.simplify()`, and `math.evaluate()` are the three top-level functions. `math.solve()` returns a `MathExpr` object — a structured result with full introspection capability.

---

## Top-Level Functions

### `math.solve(equation: txt) -> MathExpr`

Parses and solves an equation string for its variable. Returns a `MathExpr` object.

```mlang
expr = math.solve("3x + 6 = 12");
```

**Accepted equation formats:**

```
"ax + b = c"           // linear
"x^2 - 4x + 4 = 0"    // polynomial
"2y + 5 = 3y - 1"     // multi-term, single variable
"pi * r^2 = 100"       // symbolic constants in equation
```

**Operators inside strings:** `+`, `-`, `*`, `/`, `^`  
**Grouping:** `(` `)`  
**Variables:** any identifier — `x`, `y`, `r`, `temp`, `x1`  
**Constants:** numeric literals, `pi`, `e`, `tau`, `inf`

---

### `math.simplify(expr: txt) -> txt`

Reduces an expression to its simplest symbolic form. Returns a `txt` string.

```mlang
s = math.simplify("2x + x + 4 - 1");
print(s);   // "3x + 3"

s = math.simplify("(x^2 - 1) / (x - 1)");
print(s);   // "x + 1"
```

Does not require an `=` sign. Works on single-side expressions.

---

### `math.evaluate(expr: txt) -> dec`

Evaluates a numeric expression and returns a `dec`. Constants are substituted before evaluation.

```mlang
v = math.evaluate("pi * 3^2");
print(v);   // 28.274333882308138

v = math.evaluate("e^2");
print(v);   // 7.38905609893065
```

Raises a runtime error if the expression contains unresolved variables.

---

## MathExpr Object Methods

Every `math.solve()` call returns a `MathExpr`. The object is immutable — methods only read, never mutate.

---

### `.result() -> txt`

Returns the final result of the solved equation as a string.

```mlang
expr = math.solve("2x + 4 = 10");
print(expr.result());   // "x = 3"

expr = math.solve("x^2 = 16");
print(expr.result());   // "x = 4"
```

For equations with symbolic results:

```mlang
expr = math.solve("pi * r^2 = 100");
print(expr.result());   // "r = sqrt(100 / pi)"
```

---

### `.step(n: int) -> txt`

Returns the nth transformation step as a string. Steps are 0-indexed.

- Step `0` — always the original equation
- Step `n` — intermediate transformation
- Step `last` — equivalent to `.result()`

```mlang
expr = math.solve("3x + 6 = 12");
print(expr.step(0));    // "3x + 6 = 12"
print(expr.step(1));    // "3x = 6"
print(expr.step(2));    // "x = 2"
```

Accessing an out-of-range index raises a runtime error.

---

### `.steps() -> array<txt>`

Returns all transformation steps as an array of strings.

```mlang
expr = math.solve("3x + 6 = 12");
steps = expr.steps();
// → ["3x + 6 = 12", "3x = 6", "x = 2"]

for step in steps {
    print(step);
}
```

---

### `.original() -> txt`

Returns the original equation string exactly as passed to `math.solve()`.

```mlang
expr = math.solve("2x + 4 = 10");
print(expr.original());     // "2x + 4 = 10"
```

---

### `.variable() -> txt`

Returns the name of the variable that was solved for.

```mlang
expr = math.solve("3y + 9 = 0");
print(expr.variable());     // "y"

expr = math.solve("2temp = 8");
print(expr.variable());     // "temp"
```

---

## Future Methods (v0.2+)

These methods are planned and reserved. Calling them in v0.1.0 raises a runtime error with `"not yet implemented"`.

### `.derivative() -> MathExpr`

Returns a new `MathExpr` representing the derivative of the expression with respect to the solved variable.

```mlang
expr = math.solve("x^3 + 2x = 0");
d = expr.derivative();
print(d.result());   // "3x^2 + 2"
```

---

### `.integrate() -> MathExpr`

Returns a new `MathExpr` representing the indefinite integral.

```mlang
expr = math.solve("x^2 = 0");
i = expr.integrate();
print(i.result());   // "x^3 / 3 + C"
```

---

### `.expand() -> MathExpr`

Expands factored or grouped expressions.

```mlang
expr = math.solve("(x + 2)(x - 3) = 0");
e = expr.expand();
print(e.result());   // "x^2 - x - 6 = 0"
```

---

### `.factor() -> MathExpr`

Factors a polynomial expression.

```mlang
expr = math.solve("x^2 - x - 6 = 0");
f = expr.factor();
print(f.result());   // "(x + 2)(x - 3) = 0"
```

---

## Error Behavior

| Situation | Error |
|---|---|
| Equation has no variable | runtime error: `"no variable found in equation"` |
| Equation is unsolvable | runtime error: `"equation has no solution"` |
| `.step(n)` out of range | runtime error: `"step index out of range"` |
| `math.evaluate()` with variable | runtime error: `"unresolved variable in expression"` |
| Future method called | runtime error: `"not yet implemented"` |
