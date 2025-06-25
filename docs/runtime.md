# MLang Runtime Architecture

The MLang Runtime is responsible for managing the execution of MLang programs. It processes source files through multiple phases: lexing, parsing, interpreting, and executing — including handling built-in features such as math resolution, console printging, and user-defined functions.

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
[Built-in Engine]→ Console, Math Engine, prints, Errors
```

More details coming soon!

---

## 🎯 Runtime Goals

* Provide a predictable and debuggable environment
* Offer minimal core abstractions
* Prioritize simplicity, clarity, and educational value
* Allow expressive symbolic computation without complexity

---

> See [`engine.md`](engine.md) for math-specific behavior
> See [`grammar.md`](grammar.md) for syntax rules
> See [`overview.md`](overview.md) for project structure