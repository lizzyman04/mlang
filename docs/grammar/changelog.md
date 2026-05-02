# Grammar Changelog

This file tracks how the MLang grammar evolved across implementation phases. Each entry lists what was added to the formal grammar and why.

## Phase 1 — Lexer + Tokens

Defined the terminal set. No grammar productions yet — the lexer existed before the parser.

**Terminals introduced:**
- `INT_LIT`, `DEC_LIT`, `TXT_LIT`, `BOOL_LIT`
- `IDENT`
- Arithmetic symbols: `+`, `-`, `*`, `/`
- Comparison symbols: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical symbols: `&&`, `||`, `!`
- Simple symbols: `(`, `)`, `{`, `}`, `[`, `]`, `,`, `;`, `.`, `=`
- Keywords: `int`, `dec`, `txt`, `bool`, `void`, `true`, `false`
- Comments: `#` to end-of-line (stripped at lex time)

---

## Phase 2 — Variables, Arithmetic, Print

First grammar productions. Established the top-level structure and basic expressions.

**Productions added:**
```ebnf
program    = { top_level_decl } ;
var_decl   = type IDENT "=" expression ";" ;
print_stmt = "print" "(" expression ")" ";" ;
expression = or_expr ;
add_expr   = mul_expr { ("+" | "-") mul_expr } ;
mul_expr   = unary    { ("*" | "/") unary } ;
unary      = ("!" | "-") unary | primary ;
primary    = INT_LIT | DEC_LIT | TXT_LIT | BOOL_LIT | IDENT | "(" expression ")" ;
```

**Keywords added:** `print`, `main`

---

## Phase 3 — Control Flow

Added branching and looping constructs.

**Productions added:**
```ebnf
if_stmt       = "if" expression block [ "else" block ] ;
while_stmt    = "while" expression block ;
for_stmt      = "for" IDENT "in" expression ".." expression block
              | "for" IDENT "in" expression block ;
break_stmt    = "break" ";" ;
continue_stmt = "continue" ";" ;
block         = "{" { statement } "}" ;
```

**Keywords added:** `if`, `else`, `while`, `for`, `in`, `break`, `continue`

**Operators added:** `||`, `&&` (logical), full comparison set in `cmp_expr`

---

## Phase 4 — Arrays

Added array literals, indexing, and array-returning for expressions.

**Productions added/extended:**
```ebnf
type    = ... | "array" "<" type ">" ;
primary = ... | "[" [ expr_list ] "]" ;
postfix = primary { "[" expression "]" | "." IDENT [ call_args ] } ;
assign_stmt = IDENT "[" expression "]" "=" expression ";" ;
```

**Keywords added:** `array`

**Note:** Array methods (`push`, `pop`, `len`, `contains`, `remove`) are dispatched at evaluation time as postfix method calls, not encoded in the grammar as distinct productions.

---

## Phase 5 — Functions + Recursion

Added function declarations, calls, and the return statement.

**Productions added:**
```ebnf
top_level_decl = struct_decl | function_decl ;
function_decl  = type IDENT "(" [ param_list ] ")" block
               | "main" "(" ")" block ;
param_list     = param { "," param } ;
param          = type IDENT ;
return_stmt    = "return" expression ";" ;
call_args      = "(" [ expr_list ] ")" ;
primary        = ... | IDENT [ call_args | struct_init ] ;
let_decl       = "let" IDENT "=" expression ";" ;
```

**Keywords added:** `return`, `let`

**Note:** `let` introduces type-inferred local variables. `let` differs from typed `var_decl` in that the type is not written explicitly.

---

## Phase 6 — Structs + Field Access

Added user-defined aggregate types.

**Productions added:**
```ebnf
struct_decl  = "struct" IDENT "{" { struct_field } "}" ;
struct_field = type IDENT ;
struct_init  = "{" field_init { "," field_init } "}" ;
field_init   = IDENT "=" expression ;
postfix      = primary { "[" expression "]" | "." IDENT [ call_args ] } ;
assign_stmt  = ... | IDENT "." IDENT "=" expression ";" ;
```

**Keywords added:** `struct`

**Type grammar extended:**
```ebnf
type = "int" | "dec" | "txt" | "bool" | "void"
     | "array" "<" type ">"
     | IDENT ;    (* struct type *)
```

---

## Phase 7 — I/O and Type Casts

Added built-in I/O functions and type conversion syntax. No new grammar productions were required — `read`, `read_int`, `read_dec` parse as ordinary `FnCall` via `IDENT call_args`. The `int(x)`, `dec(x)`, `txt(x)` casts required parser disambiguation because `int`, `dec`, `txt` lex as `Keyword` tokens rather than identifiers.

**Grammar clarification added:**
```ebnf
cast_call = ("int" | "dec" | "txt") "(" expression ")" ;
primary   = ... | cast_call ;
```

**String coercion added (evaluation rule, not grammar):**

`TxtLiteral + IntLiteral`, `TxtLiteral + DecLiteral`, and `TxtLiteral + BoolLiteral` now produce `TxtLiteral` via the `+` operator, enabling natural string interpolation without an explicit cast.
