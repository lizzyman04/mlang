# Math Engine

MLang has a built-in symbolic math engine — no imports, no external libraries.

## Quick examples

```mlang
main() {
    # Solve equations
    print(math.solve("2x + 4 = 10").result())   # x = 3

    # Simplify expressions
    print(math.simplify("2x + x + 4 - 1"))      # 3x + 3

    # Evaluate numeric expressions
    print(math.evaluate("pi * 3^2"))             # 28.274333882308138
}
```

## Topics

- [Solving equations](/math/solve) — `math.solve()` and `MathExpr` methods
- [Simplifying expressions](/math/simplify) — `math.simplify()`
- [Evaluating expressions](/math/evaluate) — `math.evaluate()`
- [Constants](/math/constants) — `pi`, `e`, `tau`, `inf`
- [Functions](/math/functions) — `sqrt`, `sin`, `cos`, `tan`, `log`, `exp`, `abs`, `pow`
