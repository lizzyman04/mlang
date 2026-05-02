# Environment Model

`src/core/interpreter/env.rs` implements the runtime environment: variable storage, function registry, and struct schemas.

## `Environment`

```rust
pub struct Environment {
    vars:      HashMap<String, (Type, Expression)>,
    parent:    Option<Box<Environment>>,
    functions: HashMap<String, FunctionDef>,
    structs:   HashMap<String, Vec<(Type, String)>>,
}
```

### Variable storage

Variables are stored as `(Type, Expression)` pairs. The `Type` is retained for:
- Type-checking on assignment (`coerce_to_type`)
- Mutable method dispatch (push/pop need to know the inner array element type)

`get(name)` → `Option<&(Type, Expression)>`  
`set(name, type, value)` → inserts or overwrites in the current frame

### Scope chains

`child_for_call()` creates a new `Environment` with `parent` pointing to the caller's environment. This is used exclusively for function calls — not for block scopes (if/while/for bodies share the enclosing scope).

Variable lookup walks the parent chain:

```rust
pub fn get(&self, name: &str) -> Option<&(Type, Expression)> {
    self.vars.get(name).or_else(|| self.parent.as_ref()?.get(name))
}
```

### Function registry

`FunctionDef` stores the parsed body and parameter list:

```rust
pub struct FunctionDef {
    pub params:      Vec<(Type, String)>,
    pub return_type: Type,
    pub body:        Vec<ASTNode>,
}
```

`register_function(name, def)` — called during the executor's first pass  
`get_function(name)` → `Option<&FunctionDef>`

### Struct registry

`register_struct(name, fields)` stores `Vec<(Type, String)>` for each struct type.  
Currently used for schema awareness (field ordering, future type validation).

## Scoping rules summary

| Context | New scope? | Sees parent variables? |
|---|---|---|
| Function call | Yes (`child_for_call`) | No — isolated scope |
| `if`/`else` body | No | Yes — same scope |
| `while` body | No | Yes — same scope |
| `for` body | No (loop var injected into enclosing scope) | Yes |

This means variables declared inside `if`/`while`/`for` blocks are visible after the block exits — there is no block-level scoping.
