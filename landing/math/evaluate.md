# Evaluating Expressions

## `math.evaluate(expression: txt) -> dec`

Evaluates a numeric expression (no free variables). Returns a `dec` value. Supports all built-in [constants](/math/constants) and [functions](/math/functions).

```mlang
result = math.evaluate("pi * 3^2")
print(result)   # 28.274333882308138
```

## Examples

### Using constants

```mlang
main() {
    print(math.evaluate("pi * 5^2"))    # 78.53981633974483
    print(math.evaluate("2 * pi"))      # 6.283185307179586
    print(math.evaluate("e"))           # 2.718281828459045
}
```

### Using functions

```mlang
main() {
    print(math.evaluate("sqrt(16)"))    # 4.0
    print(math.evaluate("sqrt(16) + 2"))# 6.0
    print(math.evaluate("abs(-7)"))     # 7.0
}
```

### Arithmetic

```mlang
main() {
    print(math.evaluate("2 + 3 * 4"))   # 14
    print(math.evaluate("(2 + 3) * 4")) # 20
}
```

::: warning
Expressions with free variables (e.g. `"x + 1"`) return an error. Use [`math.solve()`](/math/solve) or [`math.simplify()`](/math/simplify) for symbolic work.
:::
