# MLang — Contributor Documentation

This directory contains technical documentation for contributors and language implementers. It covers the internals of the interpreter, the design decisions behind each phase, and the conventions for extending the language.

## Structure

```
docs/
├── architecture/          # End-to-end design and data flow
│   ├── overview.md        # High-level pipeline diagram and phases
│   ├── pipeline.md        # Detailed Lexer → Parser → AST → Eval flow
│   ├── ast.md             # All ASTNode and Expression variants
│   └── environment.md     # Variable scope and function registry
│
├── internals/             # Component deep-dives
│   ├── lexer.md           # Tokenizer rules and token types
│   ├── parser.md          # Statement and expression parsing strategy
│   ├── evaluator.md       # Evaluation rules and type coercion
│   └── builtins.md        # print, read, int/dec/txt, array methods
│
├── contributing/          # How to contribute
│   ├── setup.md           # Dev environment and toolchain
│   ├── adding-a-feature.md # End-to-end guide: add a language feature
│   ├── testing.md         # Example-file test conventions
│   └── conventions.md     # Commit style, code formatting, naming
│
└── grammar/               # Language specification
    ├── formal.md          # EBNF formal grammar
    └── changelog.md       # Grammar evolution across phases
```

## Quick links

| Goal | Document |
|---|---|
| Understand the big picture | [architecture/overview.md](architecture/overview.md) |
| Trace how a token becomes a value | [architecture/pipeline.md](architecture/pipeline.md) |
| Add a new statement type | [contributing/adding-a-feature.md](contributing/adding-a-feature.md) |
| Understand how scopes work | [architecture/environment.md](architecture/environment.md) |
| See the formal grammar | [grammar/formal.md](grammar/formal.md) |

## Implementation phases

| Phase | Feature | Key commits |
|---|---|---|
| 1 | Lexer + basic tokens | `feat(lexer)` |
| 2 | Variables, arithmetic, print | `feat(eval)` |
| 3 | Control flow (if/while/for) | `feat(parser)` |
| 4 | Arrays + methods | `feat(eval)` |
| 5 | Functions + recursion | `feat(eval)` |
| 6 | Structs + field access | `feat(eval)` |
| 7 | I/O: read(), type casts | `feat(builtins)` |
