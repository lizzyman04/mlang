# AST Node Reference

All AST types live in `src/core/parser/ast/`.

## `Type` (`types.rs`)

Represents the static type of a value or declaration.

```rust
pub enum Type {
    Int,
    Dec,
    Txt,
    Bool,
    Void,
    Array(Box<Type>),   // e.g. Array(Box::new(Int))
    Struct(String),     // struct name
}
```

`Type` implements `Display` for error messages and `PartialEq` for type checking.

## `Expression` (`expr.rs`)

Represents a value-producing construct. Every expression evaluates to a concrete `Expression` variant (a literal).

```rust
pub enum Expression {
    // Literals
    IntLiteral(i64),
    DecLiteral(f64),
    TxtLiteral(String),
    BoolLiteral(bool),
    ArrayLiteral(Vec<Expression>),
    StructLiteral { name: String, fields: Vec<(String, Expression)> },

    // Value producers
    Identifier(String),
    Binary   { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Unary    { operator: Token, operand: Box<Expression> },
    Logical  { left: Box<Expression>, operator: Token, right: Box<Expression> },

    // Collection access
    ArrayAccess { array: Box<Expression>, index: Box<Expression> },
    FieldAccess { object: Box<Expression>, field: String },

    // Call forms
    MethodCall { object: Box<Expression>, method: String, args: Vec<Expression> },
    FnCall     { name: String, args: Vec<Expression> },
}
```

After evaluation every expression must reduce to one of the six literal variants.

## `ASTNode` (`node.rs`)

Represents a statement or top-level declaration.

```rust
pub enum ASTNode {
    // Top-level declarations
    FunctionDecl { name: String, return_type: Type, params: Vec<(Type, String)>, body: Vec<ASTNode> },
    StructDecl   { name: String, fields: Vec<(Type, String)> },

    // Statements
    PrintStmt   { expression: Box<ASTNode> },
    ReturnStmt  { value: Box<ASTNode> },
    VarDecl     { name: String, var_type: Type, value: Box<ASTNode> },
    LetDecl     { name: String, value: Box<ASTNode> },
    VarAssign   { name: String, value: Box<ASTNode> },
    FieldAssign { object: String, field: String, value: Box<ASTNode> },
    IndexAssign { name: String, index: Box<ASTNode>, value: Box<ASTNode> },
    IfStmt      { condition: Box<ASTNode>, then_body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>> },
    WhileLoop   { condition: Box<ASTNode>, body: Vec<ASTNode> },
    ForRangeLoop { var: String, start: Box<ASTNode>, end: Box<ASTNode>, body: Vec<ASTNode> },
    ForArrayLoop { var: String, array: Box<ASTNode>, body: Vec<ASTNode> },
    Break,
    Continue,
    ExprStmt(Box<ASTNode>),

    // Wrapper used during parsing
    Expression(Expression),
}
```

`ASTNode::Expression` is a transitional wrapper — it holds an `Expression` while it is being assembled inside statement parsers. After parsing, statement variants contain `Box<ASTNode>` children that are always `ASTNode::Expression(_)` wrappers.

## `ExecutionResult` (`expr.rs`)

Returned by `execute_stmt` to communicate non-local control flow:

```rust
pub enum ExecutionResult {
    Unit,               // normal completion
    Return(Expression), // early return with a value
    Break,              // break out of nearest loop
    Continue,           // continue to next iteration
}
```
