# Math Functions

Built-in functions are available inside equation strings passed to `math.evaluate()`.

| Function | Description |
|----------|-------------|
| `sqrt(x)` | Square root |
| `abs(x)` | Absolute value |
| `pow(x, y)` | x raised to y |
| `sin(x)` | Sine (radians) |
| `cos(x)` | Cosine (radians) |
| `tan(x)` | Tangent (radians) |
| `log(x)` | Natural logarithm (base e) |
| `exp(x)` | e raised to x |

## Usage

Functions appear inside the equation string passed to `math.evaluate()`:

```mlang
main() {
    print(math.evaluate("sqrt(16)"))         # 4.0
    print(math.evaluate("abs(-42)"))         # 42.0
    print(math.evaluate("log(exp(1))"))      # 1.0
    print(math.evaluate("sin(0)"))           # 0.0
    print(math.evaluate("cos(0)"))           # 1.0
}
```
