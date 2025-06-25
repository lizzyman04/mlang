# MLang — Language Overview

**MLang** is a high-level interpreted programming language designed for evaluating mathematical expressions and teaching computational thinking. It combines simplicity, performance, and symbolic math capabilities in a beginner-friendly syntax and execution model.

---

## ✨ Core Philosophy

* **Clarity First:** Syntax prioritizes human readability and elegant structure.
* **Native Math Intelligence:** Arithmetic and algebraic solvers are built-in.
* **Minimal Standard Library:** Only essential features included—no bloat.
* **Script-First Execution:** Code runs from the `main()` function by default.
* **Symbolic and Numeric Harmony:** Natively supports both real numbers and algebraic expressions.

---

## 🧩 Language Structure

### 1. **Variable Types**

MLang uses statically-typed declarations, with support for implicit type inference planned for future releases.

* `int`: Integer values
* `dec`: Decimal (floating-point) values with high precision
* `txt`: Text strings (renamed for uniqueness and clarity)
* `bool`: Boolean values (`true` or `false`)

```mlang
int x = 10;
dec pi = 3.14;
txt name = "MLang";
bool active = false;
```

---

### 2. **Functions**

Functions are declared using an explicit return type, name, and typed parameters:

```mlang
int add(int a, int b) {
    return a + b;
}
```

They must be defined **outside** the `main` function and should use **only** the arguments passed in their parameters.

---

### 3. **Main Function**

Every MLang script starts execution from the required `main()` function:

```mlang
main() {
    print("Welcome to MLang");
}
```

---

### 4. **Control Structures**

#### **Conditional (`if`, `else`)**

```mlang
int x = 5;

if x > 3 {
    print("Greater than 3");
} else {
    print("3 or less");
}
```

#### **Loop (`for`)**

```mlang
for int i = 0; i < 5; i = i + 1 {
    print(i);
}
```

> Future versions will include support for `while`, `do-while`, `foreach`, and more.

---

## 🧠 Built-in Math Engine

The `math` module is integrated directly into the language and allows symbolic equation solving and algebraic manipulation.

```mlang
expr = math.solve("2x + 4 = 10");

print(expr.result());  // Output: 3
print(expr.step(1));   // Output: "2x = 6"
print(expr.steps());   // Output: ["2x + 4 = 10", "2x = 6", "x = 3"]
```

Also includes:

* Constants: `math.pi`, `math.e`
* Symbolic differentiation, expression simplification, and upcoming calculus features

---

## 📤 Console Output

```mlang
print("This is a console message.");
```

The `print()` function writes to standard output (`stdout`).

---

## 🧱 Runtime Design

MLang is interpreted in real-time using a three-phase model:

1. **Lexer** — Tokenizes source code
2. **Parser** — Builds an abstract syntax tree (AST)
3. **Interpreter** — Traverses and executes the AST

Mathematical operations are powered by a built-in symbolic engine, with no external dependencies required.

---

## 🤝 Ideal Use Cases

* Teaching algebra and programming in an integrated way
* Embedding lightweight math tools in custom software
* Rapid scripting for symbolic or numeric computation

---

> For grammar rules, see: [docs/grammar.md](grammar.md)
> For math engine internals, see: [docs/engine.md](engine.md)