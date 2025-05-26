# MLang — Language Overview

MLang is a high-level interpreted programming language designed for mathematical expression evaluation and educational computation. It aims to combine simplicity, performance, and symbolic math processing into a beginner-friendly syntax and runtime.

---

## ✨ Core Philosophy

- **Clarity First:** MLang prioritizes human readability and elegant syntax.
- **Built-in Math Intelligence:** Arithmetic and algebraic solvers are part of the core.
- **Minimal Standard Library:** Focused, essential operations without bloating.
- **Script-First Execution:** Executes code from the `main()` function by default.
- **Symbolic and Numeric Coexistence:** Works with real values and algebraic forms natively.

---

## 🧩 Language Structure

### 1. **Variable Types**

MLang supports statically-typed variable declarations with implicit type inference possible in the future.

- `int`: Integer values
- `dec`: Decimal/float values (high precision)
- `txt`: String values (renamed for clarity and uniqueness)
- `bool`: True or false

```mlang
int x = 10;
dec pi = 3.14159;
txt name = "MLang";
bool active = false;
````

---

### 2. **Functions**

Functions are declared using only their names (no `func` or `def` keyword).

```mlang
add(int a, int b) {
    return a + b;
}
```

---

### 3. **Main Execution Entry**

All scripts are executed starting from a required `main()` function:

```mlang
main() {
    log("Welcome to MLang");
}
```

---

### 4. **Loops (Initial Support: `for`)**

```mlang
for int i = 0; i < 5; i = i + 1 {
    log(i);
}
```

Future support for `while`, `do-while`, `foreach` is planned.

---

## 🧠 Built-in Math Engine

The `math` module is built into the language. It allows users to solve complex expressions with ease.

```mlang
expr = math.solve("2x + 4 = 10");

log(expr.result());  // Output: 3
log(expr.step(1));   // Output: "2x = 6"
log(expr.steps());   // Output: ["2x + 4 = 10", "2x = 6", "x = 3"]
```

Also includes:

* `math.pi`, `math.e`
* Support for symbolic differentiation, simplification, and future calculus features

---

## 📤 Console Output

```mlang
log("This is a user message.");
error("This is an error message.");
```

The `log()` and `error()` functions print to stdout and stderr respectively.

---

## 🧱 Runtime Design (in brief)

MLang is interpreted using:

1. **Lexer** — Tokenizes the input
2. **Parser** — Builds an AST
3. **Interpreter** — Walks and executes the AST

Math operations are handled via a symbolic engine that plugs directly into the interpreter, avoiding external dependencies.

---

## 🚧 Roadmap

* [ ] Core syntax and AST design
* [ ] Basic math engine integration
* [ ] Interactive REPL mode
* [ ] Package system for modules
* [ ] File I/O and text manipulation
* [ ] Built-in test/assert syntax

---

## 🤝 Ideal Use Cases

* Teaching algebra and programming together
* Embedded math tools in custom software
* Rapid scripting for computational tasks

---

> For grammar rules, see [docs/grammar.md](grammar.md)
> For math engine internals, see [docs/math-engine.md](math-engine.md)