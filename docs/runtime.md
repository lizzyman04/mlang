# MLang Runtime Architecture

The MLang Runtime is responsible for managing the execution of MLang programs. It processes source files through multiple phases: lexing, parsing, interpreting, and executing — including handling built-in features such as math resolution, console logging, and user-defined functions.

---

## 🧬 Runtime Pipeline

```plaintext
source.mlang
    ↓
[Lexer]          → Tokens
    ↓
[Parser]         → AST (Abstract Syntax Tree)
    ↓
[Interpreter]    → Runtime Execution (scopes, memory, functions)
    ↓
[Built-in Engine]→ Console, Math Engine, Logs, Errors
````

---

## 🧩 Core Components

| Component        | Responsibility                                                    |
| ---------------- | ----------------------------------------------------------------- |
| `lexer.rs`       | Converts source code into tokens                                  |
| `parser.rs`      | Builds an AST from the token stream                               |
| `interpreter.rs` | Walks the AST, manages scope, executes statements and expressions |
| `math.rs`        | Handles symbolic math logic and resolution (`math.solve`)         |
| `utils.rs`       | Provides helper functions, constants, and formatting utilities    |
| `main.rs`        | Entry point for compiling and executing `.mlang` source files     |

---

## 📦 Memory Model

MLang uses a scoped variable memory model with block-level visibility:

```mlang
int x = 5;

my_func() {
  int x = 10;
  log(x);   // prints 10
}

log(x);     // prints 5
```

Each function call creates a new execution frame with its own scope.

---

## 🗂️ Scope Management

* **Global Scope:** variables declared outside any function
* **Function Scope:** isolated per call
* **Loop Scope:** loop bodies have isolated variable state (coming soon)

Runtime will **shadow** variables correctly and remove them after execution completes.

---

## 🧮 Built-in Math Engine

MLang programs can access symbolic math directly:

```mlang
expr = math.solve("2x + 3 = 9");
log(expr.result()); // → 3
```

The `math` engine is bound to the global runtime scope and integrated at the interpreter level.

---

## 📚 Functions

### Declaration

```mlang
my_func(int a, int b) {
  log(a + b);
}
```

### Execution

```mlang
my_func(2, 3); // prints 5
```

Functions are first-class values in the runtime and support recursion, default values (coming soon), and shadowing.

---

## 🔥 Main Execution Flow

Only the `main()` function is executed automatically. All other functions are user-invoked.

```mlang
main() {
  log("MLang is running");
}
```

The runtime starts at `main()` and halts on return or crash.

---

## 🛠️ Built-in Functions

| Name                | Description                       |
| ------------------- | --------------------------------- |
| `log()`             | Prints to console                 |
| `error()`           | Prints an error message and halts |
| `math.solve()`      | Solves algebraic expressions      |
| `math.pi`, `math.e` | Built-in constants                |

All built-in functions are implemented internally in Rust.

---

## ❌ Error Handling

Runtime errors are classified into:

* **Syntax Errors** (during parsing)
* **Runtime Errors** (e.g., undefined variable, division by zero)
* **Math Engine Errors** (e.g., malformed expressions)

Errors are displayed with:

* Filename
* Line number
* Description

Example:

```plaintext
Runtime Error at line 4: Variable 'y' is undefined
```

---

## 🔁 Loops (Phase 1: For-Loop)

Only `for` loops are supported in the initial version.

```mlang
for int i = 0; i < 5; i = i + 1 {
  log(i);
}
```

Loop variables exist within their own scope.

---

## 🔌 Integration Points

### File Loader

`.mlang` files are loaded and parsed on launch.

```bash
$ mlang run examples/hello.mlang
```

### CLI Runtime

The binary `mlang` will handle CLI execution:

* `run`: execute a source file
* `check`: perform syntax checking
* `repl` (coming soon): interactive mode

---

## 🧪 Future Runtime Features

| Feature                         | Status         |
| ------------------------------- | -------------- |
| REPL console                    | 🧪 Planned     |
| Loop extensions (`while`, `do`) | 🔄 In progress |
| Function return values          | 🚧 Partial     |
| Import multiple files/modules   | 🧪 Planned     |
| Standard Library Support        | 🧪 Planned     |
| Performance Optimizations       | 🔄 Phase 2     |

---

## 🧵 Runtime Threading Model

MLang executes in a **single-threaded synchronous model** for simplicity and determinism.

Planned upgrades include:

* Asynchronous task scheduler
* Background math solver
* Coroutine-style `yield` system

---

## 📁 Runtime Filesystem Layout

```plaintext
mlang/
├── src/
│   ├── main.rs          // Entry point
│   ├── lexer.rs         // Tokenizer
│   ├── parser.rs        // AST builder
│   ├── interpreter.rs   // Executor
│   ├── math.rs          // Math engine
│   └── utils.rs         // Common helpers
├── examples/hello.mlang // Example program
├── docs/runtime.md      // This file
```

---

## 🎯 Runtime Goals

* Provide a predictable and debuggable environment
* Offer minimal core abstractions
* Prioritize simplicity, clarity, and educational value
* Allow expressive symbolic computation without complexity

---

> See [`math-engine.md`](math-engine.md) for math-specific behavior
> See [`grammar.md`](grammar.md) for syntax rules
> See [`overview.md`](overview.md) for project structure