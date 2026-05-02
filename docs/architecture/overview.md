# Architecture Overview

MLang is a single-pass tree-walking interpreter written in Rust. There are no bytecode, JIT, or optimisation passes — source code is parsed once into an AST and immediately evaluated.

## Pipeline

```
 Source text (.mth file)
        │
        ▼
  ┌───────────┐
  │   Lexer   │  src/core/lexer/
  └─────┬─────┘
        │  Vec<Token>
        ▼
  ┌───────────┐
  │  Parser   │  src/core/parser/
  └─────┬─────┘
        │  Vec<ASTNode>
        ▼
  ┌──────────────┐
  │  Executor    │  src/core/interpreter/executor.rs
  │  (two-pass)  │
  └─────┬────────┘
        │  first pass: register structs + functions
        │  second pass: execute main()
        ▼
  ┌──────────────┐
  │  Evaluator   │  src/core/interpreter/eval.rs
  │  + Stmt exec │  src/core/interpreter/stmt.rs
  └──────────────┘
```

## Module map

```
src/
├── lib.rs                        # crate root, re-exports core + builtins
├── bin/mlang.rs                  # CLI entry point
│
├── core/
│   ├── lexer/
│   │   ├── token.rs              # Token and TokenKind
│   │   ├── rules.rs              # is_keyword(), is_variable_type(), infer_type()
│   │   ├── symbol/               # SimpleSymbol, ComparisonSymbol, MathSymbol
│   │   └── tokenizer/            # dispatch, identifiers, numbers, strings, symbols
│   │
│   ├── parser/
│   │   ├── ast/
│   │   │   ├── expr.rs           # Expression enum + ExecutionResult
│   │   │   ├── node.rs           # ASTNode enum
│   │   │   └── types.rs          # Type enum
│   │   ├── parse/
│   │   │   ├── parser.rs         # Parser struct (cursor over Vec<Token>)
│   │   │   ├── entry.rs          # top-level parse loop
│   │   │   └── expr.rs           # Pratt-style expression parser
│   │   └── stmt/
│   │       ├── parser.rs         # statement dispatch
│   │       ├── decider.rs        # type-keyword → var or fn decl
│   │       ├── func.rs           # function declaration parser
│   │       ├── var.rs            # variable declaration parser
│   │       ├── loops.rs          # for/while/if parsers
│   │       ├── print.rs          # print() statement parser
│   │       ├── return.rs         # return statement parser
│   │       └── structs.rs        # struct declaration parser
│   │
│   └── interpreter/
│       ├── env.rs                # Environment (variable + function registry)
│       ├── eval.rs               # evaluate(Expression) → Expression
│       ├── stmt.rs               # execute_stmt(ASTNode) → ExecutionResult
│       └── executor.rs           # execute(program) + repl entry points
│
└── builtins/
    └── math/                     # symbolic math engine (future)
```

## Key invariants

1. **Evaluation is pure** — `evaluate()` takes an `Expression` and an `Environment` reference, returns a new `Expression`. It never modifies the AST.
2. **Statements drive side effects** — all I/O, variable mutation, and control flow happen in `execute_stmt()`.
3. **Two-pass execution** — the executor makes one pass over the top-level AST to register struct schemas and function definitions, then a second pass to run `main()`. This allows forward references anywhere in the file.
4. **Built-ins are intercepted first** — `evaluate()` checks the function name against known built-ins (`print` is a statement, `read`/`int`/`dec`/`txt` are `FnCall` variants) before consulting the user-function registry.
