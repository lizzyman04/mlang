# Math Constants

Built-in constants are available inside equation strings passed to `math.solve()`, `math.simplify()`, and `math.evaluate()`.

| Constant | Value |
|----------|-------|
| `pi` | 3.141592653589793 |
| `e` | 2.718281828459045 |
| `tau` | 6.283185307179586 |
| `inf` | ∞ |

## Usage

Constants appear inside the equation string, not as separate identifiers:

```mlang
main() {
    print(math.evaluate("pi * 3^2"))   # 28.274333882308138
    print(math.evaluate("tau / 2"))    # 3.141592653589793
    print(math.evaluate("e"))          # 2.718281828459045
}
```
