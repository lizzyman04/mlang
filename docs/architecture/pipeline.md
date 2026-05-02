# Execution Pipeline

This document traces the journey of a single source file from disk to output.

## Step 1 — Read source

`src/bin/mlang.rs` reads the file into a `String` and passes it to the lexer.

## Step 2 — Lex

`src/core/lexer/tokenizer/mod.rs` iterates over the source characters and produces a `Vec<Token>`. Each `Token` carries:

- `kind: TokenKind` — the token variant (keyword, identifier, literal, symbol…)
- `line: usize` — for error messages
- `column: usize` — for error messages

The tokenizer dispatches on the first character of each token to a specialised sub-module:

| Module | Handles |
|---|---|
| `identifiers.rs` | Identifiers, keywords, bool literals |
| `numbers.rs` | `Int` and `Dec` literals |
| `strings.rs` | `Txt` (string) literals |
| `symbols.rs` | All operators and punctuation |

Unknown characters produce an error with line/column context.

## Step 3 — Parse

`src/core/parser/parse/entry.rs` loops over the token stream and calls `parse_statement()` until EOF. The result is a `Vec<ASTNode>` — the program's AST.

**Statement dispatch** (`stmt/parser.rs`):

The parser peeks at the current token and routes to a specialised sub-parser:

- `main` keyword → `parse_function_decl`
- `print` keyword → `parse_print_stmt`
- `return` keyword → `parse_return_stmt`
- `if` / `while` / `for` → loop/conditional parsers
- `let` → `parse_let_decl`
- `struct` → `parse_struct_decl`
- variable type keyword (`int`, `dec`, …) → `parse_var_or_function_decl`
- identifier → `parse_ident_stmt` (assignment, method call, or fn call)

**Expression parsing** (`parse/expr.rs`):

Expressions use a recursive descent parser with explicit precedence levels:

```
parse_expression
  └─ parse_or (||)
       └─ parse_and (&&)
            └─ parse_comparison (==, !=, <, >, <=, >=)
                 └─ parse_term (+, -)
                      └─ parse_factor (*, /)
                           └─ parse_unary (!, -)
                                └─ parse_postfix ([], .)
                                     └─ parse_primary (literal, ident, fn call, …)
```

## Step 4 — Execute

`executor.rs::execute()` takes the top-level `Vec<ASTNode>` and:

1. **First pass** — iterates the program and registers every `StructDecl` and non-`main` `FunctionDecl` into the `Environment`.
2. **Second pass** — finds the `main` function and runs its body through `execute_stmt()`.

## Step 5 — Evaluate statements

`stmt.rs::execute_stmt()` dispatches on the `ASTNode` variant:

- `PrintStmt` → evaluates the inner expression, formats it, writes to stdout (or a capture buffer for tests).
- `VarDecl` / `LetDecl` → evaluates the RHS, coerces to the declared type, stores in `env`.
- `VarAssign` / `FieldAssign` / `IndexAssign` → update an existing binding.
- `IfStmt` / `WhileLoop` / `ForRangeLoop` / `ForArrayLoop` → recursive `execute_stmt` on the selected branch body.
- `ReturnStmt` → evaluates expression and returns `ExecutionResult::Return(value)`.
- `ExprStmt` → evaluates expression for side effects (e.g., a standalone function call).

## Step 6 — Evaluate expressions

`eval.rs::evaluate()` recursively reduces an `Expression` to a literal value:

- Literals pass through unchanged.
- `Identifier` → lookup in `env`.
- `Binary` / `Unary` / `Logical` → operator evaluation.
- `ArrayAccess` / `FieldAccess` → index/field extraction.
- `MethodCall` → array methods (len, push, pop, …).
- `StructLiteral` → recursively evaluate each field expression.
- `FnCall` → built-in dispatch first, then user function via `env.get_function()`.

The return value propagates back up through `execute_stmt` until it is consumed by the nearest enclosing function body.
