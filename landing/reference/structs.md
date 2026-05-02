# Structs

Structs are named value types with typed fields. They are defined at the top level and can be used as variable types, parameter types, and return types.

## Definition

```mlang
struct Point {
    int x
    int y
}
```

Fields are separated by newlines (no semicolons inside struct bodies).

## Instantiation

```mlang
let p = Point { x = 10, y = 20 };
```

Or with an explicit type:

```mlang
Point origin = Point { x = 0, y = 0 };
```

## Field access

```mlang
print(p.x);   # 10
print(p.y);   # 20
```

## Field mutation

```mlang
p.x = 99;
print(p.x);   # 99
```

## Nested structs

```mlang
struct Rect {
    Point top_left
    Point bottom_right
}

main() {
    let r = Rect {
        top_left     = Point { x = 0, y = 10 },
        bottom_right = Point { x = 10, y = 0 }
    };
    print(r.top_left.y);       # 10
    print(r.bottom_right.x);   # 10
}
```

## Structs in functions

Structs can be passed to and returned from functions:

```mlang
Point translate(Point p, int dx, int dy) {
    return Point { x = p.x + dx, y = p.y + dy };
}

main() {
    let p = Point { x = 1, y = 1 };
    let q = translate(p, 5, 3);
    print(q.x);   # 6
    print(q.y);   # 4
}
```

## Structs are value types

Structs are copied on assignment and when passed to functions. Mutations inside a function do not affect the caller's copy.

## Struct registration

All struct declarations are registered before execution begins, so structs can be referenced anywhere in the file regardless of declaration order.
