# MLang Math Engine

The MLang Math Engine is an integrated symbolic and numeric computation system embedded directly into the language runtime. Its goal is to make algebraic manipulation, equation solving, and math automation accessible without external libraries or import statements.

---

## 🎯 Purpose

Unlike traditional programming languages that rely on third-party math libraries, MLang treats algebraic problem-solving as a first-class citizen.

This allows:

- Solving linear equations
- Tracking resolution steps
- Supporting symbolic variables and constants
- Exposing the result as an introspectable object

---

## 🧠 Core Syntax

### Solving Equations

```mlang
expr = math.solve("2x + 4 = 10");
````

### Accessing Results

```mlang
log(expr.result());    // → 3
log(expr.step(1));     // → "2x = 6"
log(expr.steps());     // → ["2x + 4 = 10", "2x = 6", "x = 3"]
```

---

## 🔍 Expression Format

Equations must be passed as strings:

```mlang
"ax + b = c"
"(x^2 + 3x) = 0"
"2y + 5 = 3y - 1"
```

Accepted syntax:

* **Operators:** `+`, `-`, `*`, `/`, `^`
* **Grouping:** `(` `)`
* **Equality:** `=`
* **Variables:** any valid identifier (`x`, `y1`, `temp`)
* **Constants:** real or integer numbers

---

## 🏗️ Architecture

Internally, the math engine has 4 core stages:

### 1. **Lexer**

Tokenizes math strings into meaningful components.

Example:

```text
"2x + 4 = 10" → [2, x, +, 4, =, 10]
```

### 2. **Parser**

Converts tokens into an Abstract Syntax Tree (AST):

```
       =
      / \
   +     10
  / \
2x   4
```

### 3. **Transformer**

* Rearranges the equation
* Isolates variables
* Reduces terms
* Collects like terms and simplifies expressions

### 4. **Solver**

* Solves algebraically (symbolic)
* Optionally evaluates numerically
* Records steps for introspection

---

## 🔬 Resolution Object API

Every `math.solve()` call returns an expression object with the following API:

| Method       | Description                                 |
| ------------ | ------------------------------------------- |
| `result()`   | Returns the final numeric or symbolic value |
| `step(n)`    | Returns the nth step in the resolution      |
| `steps()`    | Returns an array of all steps               |
| `original()` | Returns the original equation               |
| `variable()` | Returns the variable being solved for       |

> Future extensions will include `.derivative()`, `.integrate()`, `.simplify()`, etc.

---

## 🧪 Examples

### Linear Equation

```mlang
expr = math.solve("3x + 6 = 12");
log(expr.steps());
// → ["3x + 6 = 12", "3x = 6", "x = 2"]
```

### Quadratic Equation (basic support)

```mlang
expr = math.solve("x^2 - 4x + 4 = 0");
log(expr.result());
// → x = 2
```

> Advanced support for multiple roots will be added in version 0.2+

---

## 🧩 Symbolic Constants

MLang supports symbolic constants directly:

* `math.pi` → π
* `math.e` → Euler’s number
* `math.inf` → ∞

```mlang
expr = math.solve("pi * r^2 = 100");
log(expr.result()); // r = sqrt(100 / pi)
```

---

## 🔄 Planned Features

| Feature                      | Status         |
| ---------------------------- | -------------- |
| Linear equation solving      | ✅ Implemented  |
| Step tracking                | ✅ Implemented  |
| Symbolic constants           | ✅ Implemented  |
| Quadratic equations          | 🔄 Partial     |
| Simplification (`.simplify`) | 🚧 In progress |
| Derivation (`.derivative`)   | 🚧 In progress |
| Integration (`.integrate`)   | 🧪 Planned     |
| Trigonometric support        | 🧪 Planned     |
| Multi-variable equations     | 🧪 Future      |

---

## 🧱 Engine Stack (Implementation Overview)

| Layer          | Description                            |
| -------------- | -------------------------------------- |
| **Lexer**      | Tokenizes math string inputs           |
| **Parser**     | Builds the AST                         |
| **Normalizer** | Rewrites expressions in canonical form |
| **Solver**     | Executes algebraic solution            |
| **Renderer**   | Formats steps and output strings       |

Implemented in `src/math.rs`, integrated with the interpreter runtime.

---

## 🚀 Goals

* Build a beginner-friendly algebra environment
* Serve as a core utility for interactive tutorials
* Enable symbolic computation on the fly
* Remove the need for external CAS (Computer Algebra Systems)

---

## 📌 Design Notes

* Math operations are lazy-evaluated until `.result()` is called
* All steps are stored in memory as snapshots of the transformation
* Constants and known functions (`sqrt`, `log`, `abs`) are internally mapped

---

> See [`runtime.md`](runtime.md) for how the engine integrates with the interpreter
> See [`grammar.md`](grammar.md) for syntax rules and language embedding