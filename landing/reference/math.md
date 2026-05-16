# Math Engine

MLang has a built-in symbolic math engine for solving equations, simplifying expressions, and numeric evaluation.

## Functions

### `math.solve(equation: txt) -> MathExpr`

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

### `math.simplify(expression: txt) -> txt`

Reduces an expression to its simplest form.

```mlang
result = math.simplify("2x + x + 4 - 1")
print(result)   # 3x + 3
```

### `math.evaluate(expression: txt) -> dec`

Evaluates a numeric expression (no variables).

```mlang
result = math.evaluate("pi * 3^2")
print(result)   # 28.274333882308138

result = math.evaluate("sqrt(16) + 2")
print(result)   # 6.0
```

## MathExpr Object Methods

When you call `math.solve()`, you get a `MathExpr` object with these methods:

| Method | Returns | Description |
|--------|---------|-------------|
| `.result()` | `txt` | Final solution |
| `.step(n)` | `txt` | Nth transformation step (0-indexed) |
| `.steps()` | `array<txt>` | All transformation steps |
| `.original()` | `txt` | Original equation |
| `.variable()` | `txt` | Variable that was solved for |

## Constants

| Constant | Value |
|----------|-------|
| `math.pi` | 3.141592653589793 |
| `math.e` | 2.718281828459045 |
| `math.tau` | 6.283185307179586 |
| `math.inf` | ∞ |

## Functions

| Function | Description |
|----------|-------------|
| `math.sqrt(x)` | Square root |
| `math.abs(x)` | Absolute value |
| `math.pow(x, y)` | x raised to y |
| `math.sin(x)`, `math.cos(x)`, `math.tan(x)` | Trigonometric (radians) |
| `math.log(x)` | Natural logarithm |
| `math.exp(x)` | e raised to x |

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
    print(expr.result())   # x = 3, x = 2
}
```

### Using constants

```mlang
main() {
    area = math.evaluate("pi * 5^2")
    print(area)   # 78.53981633974483
}
```
