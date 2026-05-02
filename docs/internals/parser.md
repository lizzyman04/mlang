# Parser Internals

Source: `src/core/parser/`

## Parser struct

`parse/parser.rs` wraps a `Vec<Token>` with a cursor:

```rust
pub struct Parser {
    tokens:  Vec<Token>,
    current: usize,
}
```

Key methods:
- `peek()` — current token without advancing
- `peek_ahead(n)` — look n tokens ahead without advancing
- `advance()` — consume and return current token
- `consume(kind)` — advance and assert the token matches `kind`
- `check(kind)` — peek equality without advancing

## Statement parsing

`stmt/parser.rs::parse_statement()` dispatches on the current token:

```
Keyword("main")          → parse_function_decl
Keyword("print")         → parse_print_stmt
Keyword("return")        → parse_return_stmt
Keyword("if")            → parse_if_stmt
Keyword("while")         → parse_while_loop
Keyword("for")           → parse_for_loop
Keyword("break")         → ASTNode::Break
Keyword("continue")      → ASTNode::Continue
Keyword("let")           → parse_let_decl
Keyword("struct")        → parse_struct_decl
Keyword(variable_type)   → parse_var_or_function_decl  (or cast call if followed by `(`)
Identifier               → parse_ident_stmt
```

### Type keyword disambiguation

When a type keyword (`int`, `dec`, `txt`) appears in statement position, `peek_ahead(1)` distinguishes three cases:

| Next token  | Interpretation | Handler |
|---|---|---|
| `(`         | Cast call: `int(x);` | Parsed as `ExprStmt(FnCall)` |
| `Identifier` then `(` | Function decl: `int add(…)` | `parse_function_decl_with_signature` |
| `Identifier` then `=` | Variable decl: `int x = …` | `parse_var_decl_with_name_and_type` |

### Identifier statement disambiguation

`parse_ident_stmt` reads the identifier name then peeks the next token:

| Next token | Interpretation |
|---|---|
| `(`  | Standalone function call |
| `[`  | Array index assignment |
| `.`  then `(` | Method call |
| `.`  then `=` | Field assignment |
| `=`  | Variable assignment |

## Expression parsing

`parse/expr.rs` uses recursive descent with explicit precedence levels (lowest to highest):

1. `parse_or` — `||`
2. `parse_and` — `&&`
3. `parse_comparison` — `==` `!=` `<` `>` `<=` `>=`
4. `parse_term` — `+` `-`
5. `parse_factor` — `*` `/`
6. `parse_unary` — `!` unary `-`
7. `parse_postfix` — `[]` `.`
8. `parse_primary` — literals, identifiers, parenthesised expressions, array literals, struct literals, function calls, type casts

### `parse_primary` special cases

- `Identifier` followed by `(` → `FnCall`
- `Identifier` followed by `{ ident =` → `StructLiteral`
- `Keyword("int"|"dec"|"txt")` followed by `(` → cast `FnCall`
- `[` → `ArrayLiteral`
- `(` → grouped expression (precedence override)

## Function and struct declaration parsing

`stmt/func.rs::parse_function_decl_with_signature` handles the parameter list and body. `parse_type_annotation` handles both primitive type keywords and struct-typed parameters (the latter appear as `Identifier` tokens and become `Type::Struct(name)`).

`stmt/structs.rs::parse_struct_decl` reads field definitions as `type field_name` pairs separated by newlines (no semicolons inside struct bodies).
