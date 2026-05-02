# Functions

## Declaration

```mlang
<return_type> <name>(<type> <param>, ...) {
    # body
    return <expr>;
}
```

Functions are declared at the top level, outside `main`. All parameters and the return type are explicitly typed.

```mlang
int add(int a, int b) {
    return a + b;
}

txt greet(txt name) {
    return "Hello, " + name;
}

void log(txt msg) {
    print(msg);
}
```

## Calling functions

```mlang
main() {
    print(add(3, 4));       # 7
    print(greet("MLang"));  # Hello, MLang
    log("done");
}
```

## Return types

| Type          | Meaning                              |
|---------------|--------------------------------------|
| `int`         | Returns a 64-bit integer             |
| `dec`         | Returns a decimal                    |
| `txt`         | Returns a string                     |
| `bool`        | Returns a boolean                    |
| `array<T>`    | Returns an array                     |
| `StructName`  | Returns an instance of a struct      |
| `void`        | No return value                      |

A `void` function may omit `return`, or use `return;` (not yet implemented — just omit it).

## Recursion

Functions can call themselves directly:

```mlang
int factorial(int n) {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

int fib(int n) {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
```

## Struct-typed parameters and return values

```mlang
struct Vec2 {
    dec x
    dec y
}

Vec2 scale(Vec2 v, dec factor) {
    return Vec2 { x = v.x * factor, y = v.y * factor };
}

main() {
    let v = Vec2 { x = 1.0, y = 2.0 };
    let doubled = scale(v, 2.0);
    print(doubled.x);   # 2.0
}
```

## Scope

- Functions have their own isolated scope.
- Parameters are passed by value (structs and arrays are copied).
- Functions cannot access variables from the caller's scope.

## Forward references

All non-`main` functions and struct declarations are registered before execution begins, so declaration order does not matter.
