# Formal Grammar

EBNF notation. `{ }` = zero or more, `[ ]` = optional, `|` = alternation.

## Program

```ebnf
program       = { top_level_decl } ;
top_level_decl = struct_decl | function_decl ;
```

## Declarations

```ebnf
struct_decl   = "struct" IDENT "{" { struct_field } "}" ;
struct_field  = type IDENT ;

function_decl = type IDENT "(" [ param_list ] ")" block
              | "main" "(" ")" block ;
param_list    = param { "," param } ;
param         = type IDENT ;
```

## Types

```ebnf
type = "int" | "dec" | "txt" | "bool" | "void"
     | "array" "<" type ">"
     | IDENT ;          (* struct type *)
```

## Statements

```ebnf
block      = "{" { statement } "}" ;

statement  = print_stmt
           | return_stmt
           | if_stmt
           | while_stmt
           | for_stmt
           | break_stmt
           | continue_stmt
           | var_decl
           | let_decl
           | assign_stmt
           | expr_stmt ;

print_stmt    = "print" "(" expression ")" ";" ;
return_stmt   = "return" expression ";" ;
if_stmt       = "if" expression block [ "else" block ] ;
while_stmt    = "while" expression block ;
for_stmt      = "for" IDENT "in" expression ".." expression block
              | "for" IDENT "in" expression block ;
break_stmt    = "break" ";" ;
continue_stmt = "continue" ";" ;
var_decl      = type IDENT "=" expression ";" ;
let_decl      = "let" IDENT "=" expression ";" ;
assign_stmt   = IDENT "=" expression ";"
              | IDENT "[" expression "]" "=" expression ";"
              | IDENT "." IDENT "=" expression ";" ;
expr_stmt     = expression ";" ;
```

## Expressions

Listed from lowest to highest precedence:

```ebnf
expression   = or_expr ;
or_expr      = and_expr { "||" and_expr } ;
and_expr     = cmp_expr { "&&" cmp_expr } ;
cmp_expr     = add_expr { cmp_op add_expr } ;
add_expr     = mul_expr { ("+" | "-") mul_expr } ;
mul_expr     = unary    { ("*" | "/") unary } ;
unary        = ("!" | "-") unary | postfix ;
postfix      = primary { "[" expression "]" | "." IDENT [ call_args ] } ;
primary      = INT_LIT | DEC_LIT | TXT_LIT | BOOL_LIT
             | "[" [ expr_list ] "]"
             | "(" expression ")"
             | IDENT [ call_args | struct_init ]
             | cast_call ;

call_args    = "(" [ expr_list ] ")" ;
struct_init  = "{" field_init { "," field_init } "}" ;
field_init   = IDENT "=" expression ;
cast_call    = ("int" | "dec" | "txt") "(" expression ")" ;
expr_list    = expression { "," expression } ;
```

## Comparison operators

```ebnf
cmp_op = "==" | "!=" | "<" | ">" | "<=" | ">=" ;
```

## Terminals

```ebnf
INT_LIT  = ["-"] digit { digit } ;
DEC_LIT  = ["-"] digit { digit } "." digit { digit } ;
TXT_LIT  = '"' { any_char } '"' ;
BOOL_LIT = "true" | "false" ;
IDENT    = (letter | "_") { letter | digit | "_" } ;
```

## Comments

```ebnf
comment = "#" { any_char_except_newline } newline ;
```

Comments are stripped by the lexer and do not appear in the token stream.
