# Math Constants

MLang exposes four built-in mathematical constants through the `math` namespace. Constants are available both as `math.<name>` values in MLang code and as identifiers inside equation strings passed to `math.solve()`, `math.simplify()`, and `math.evaluate()`.

---

## Available Constants

### `math.pi`

The ratio of a circle's circumference to its diameter.

```
math.pi → 3.141592653589793
```

```mlang
print(math.pi);                         // 3.141592653589793
area = math.evaluate("pi * 5^2");
print(area);                            // 78.53981633974483

expr = math.solve("pi * r^2 = 100");
print(expr.result());                   // r = sqrt(100 / pi)
```

---

### `math.e`

Euler's number — base of the natural logarithm.

```
math.e → 2.718281828459045
```

```mlang
print(math.e);                          // 2.718281828459045
v = math.evaluate("e^3");
print(v);                               // 20.085536923187668

expr = math.solve("e^x = 10");
print(expr.result());                   // x = log(10)
```

---

### `math.tau`

Tau — equal to `2 * pi`. Represents one full turn in radians.

```
math.tau → 6.283185307179586
```

```mlang
print(math.tau);                        // 6.283185307179586
full_circle = math.evaluate("tau * r");
```

---

### `math.inf`

Positive infinity. Useful for bounds, limits, and symbolic expressions.

```
math.inf → ∞
```

```mlang
print(math.inf);                        // Infinity

expr = math.solve("1/x = 0");
print(expr.result());                   // x = inf
```

---

## Constants Inside Equation Strings

Constants can appear by name inside any string passed to the engine. They are substituted before parsing.

```mlang
math.evaluate("pi + e");               // 5.859874482048838
math.evaluate("tau / 2");              // 3.141592653589793 (= pi)
math.simplify("2 * pi * r");           // "tau * r"
math.solve("e^x = tau");
```

---

## Type

All constants resolve to `dec` when used in MLang code directly. Inside equation strings they are substituted as their exact decimal representation before the engine processes the string.

```mlang
x = math.pi;    // x is dec
print(x);       // 3.141592653589793
```
