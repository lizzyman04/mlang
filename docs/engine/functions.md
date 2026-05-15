# Math Functions

MLang provides a set of numeric utility functions under the `math` namespace. These operate on `dec` values and return `dec`. They can be used directly in MLang code or as identifiers inside expression strings passed to the engine.

---

## Function Reference

### `math.sqrt(x: dec) -> dec`

Square root of `x`. Raises a runtime error if `x < 0`.

```mlang
print(math.sqrt(9));        // 3.0
print(math.sqrt(2));        // 1.4142135623730951
print(math.sqrt(0));        // 0.0
```

---

### `math.abs(x: dec) -> dec`

Absolute value of `x`.

```mlang
print(math.abs(-5));        // 5.0
print(math.abs(3.14));      // 3.14
print(math.abs(0));         // 0.0
```

---

### `math.pow(x: dec, y: dec) -> dec`

Raises `x` to the power of `y`. Equivalent to `x^y` inside equation strings.

```mlang
print(math.pow(2, 10));     // 1024.0
print(math.pow(9, 0.5));    // 3.0  (= sqrt(9))
print(math.pow(2, -1));     // 0.5
```

---

## Trigonometric Functions

All trig functions operate in **radians**.

### `math.sin(x: dec) -> dec`

```mlang
print(math.sin(0));             // 0.0
print(math.sin(math.pi / 2));   // 1.0
print(math.sin(math.pi));       // ~0.0 (floating point)
```

---

### `math.cos(x: dec) -> dec`

```mlang
print(math.cos(0));             // 1.0
print(math.cos(math.pi));       // -1.0
print(math.cos(math.pi / 2));   // ~0.0
```

---

### `math.tan(x: dec) -> dec`

Raises a runtime error at values where tangent is undefined (e.g., `pi/2`).

```mlang
print(math.tan(0));             // 0.0
print(math.tan(math.pi / 4));   // 1.0
```

---

## Exponential and Logarithmic

### `math.exp(x: dec) -> dec`

Returns `e^x`. Equivalent to `math.pow(math.e, x)`.

```mlang
print(math.exp(0));     // 1.0
print(math.exp(1));     // 2.718281828459045
print(math.exp(2));     // 7.38905609893065
```

---

### `math.log(x: dec) -> dec`

Natural logarithm (base `e`) of `x`. Raises a runtime error if `x <= 0`.

```mlang
print(math.log(1));         // 0.0
print(math.log(math.e));    // 1.0
print(math.log(math.e^2));  // 2.0
```

---

## Usage Inside Expression Strings

All functions are also valid inside strings passed to `math.evaluate()` and `math.simplify()`.

```mlang
math.evaluate("sqrt(16) + abs(-3)");        // 7.0
math.evaluate("sin(pi / 6)");               // 0.5
math.evaluate("log(e^5)");                  // 5.0
math.evaluate("pow(2, 8)");                 // 256.0
```

Inside `math.solve()` equations they appear as symbolic functions and are preserved in step output:

```mlang
expr = math.solve("sqrt(x) = 4");
print(expr.steps());
// → ["sqrt(x) = 4", "x = 16"]
```

---

## Error Behavior

| Call | Error |
|---|---|
| `math.sqrt(-1)` | runtime error: `"sqrt of negative number"` |
| `math.log(0)` | runtime error: `"log of zero or negative"` |
| `math.log(-1)` | runtime error: `"log of zero or negative"` |
| `math.tan(pi/2)` | runtime error: `"tan undefined at this value"` |

---

## Future Functions (v0.2+)

Planned additions — reserved names, not yet callable:

| Function | Description |
|---|---|
| `math.ceil(x)` | Round up to nearest integer |
| `math.floor(x)` | Round down to nearest integer |
| `math.round(x)` | Round to nearest integer |
| `math.min(a, b)` | Minimum of two values |
| `math.max(a, b)` | Maximum of two values |
| `math.asin(x)` | Inverse sine |
| `math.acos(x)` | Inverse cosine |
| `math.atan(x)` | Inverse tangent |
| `math.log10(x)` | Base-10 logarithm |
| `math.log2(x)` | Base-2 logarithm |
