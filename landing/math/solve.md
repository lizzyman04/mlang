# Solving Equations

## `math.solve(equation: txt) -> MathExpr`

Solves an equation for its variable. Returns a `MathExpr` object.

```mlang
expr = math.solve("2x + 4 = 10")
print(expr.result())   # x = 3

expr = math.solve("x^2 - 5x + 6 = 0")
print(expr.result())   # x = 3, x = 2
```

**Supported equation formats:**
- Linear: `"2x + 4 = 10"`, `"3x - 12 = 0"`, `"x/2 = 5"`
- Quadratic: `"x^2 - 5x + 6 = 0"`, `"x^2 - 4 = 0"`
- With constants: `"pi * r^2 = 100"`

## MathExpr Object Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.result()` | `txt` | Final solution string |
| `.step(n)` | `txt` | Nth transformation step (0-indexed) |
| `.steps()` | `array<txt>` | All transformation steps |
| `.original()` | `txt` | Original equation string |
| `.variable()` | `txt` | Variable that was solved for |

## Examples

### Linear equation with step-by-step

```mlang
main() {
    expr = math.solve("3x + 6 = 12")

    for i in 0..expr.steps().len() {
        print(expr.step(i))
    }
    # Step 0: 3x + 6 = 12
    # Step 1: 3x = 6
    # Step 2: x = 2
}
```

### Quadratic formula

```mlang
main() {
    expr = math.solve("x^2 - 5x + 6 = 0")
    print(expr.result())    # x = 3, x = 2
    print(expr.original())  # x^2 - 5x + 6 = 0
    print(expr.variable())  # x
}
```
