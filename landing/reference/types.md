# Types

MLang is statically typed. Every variable has a known type at declaration time.

## Primitive types

### `int`

64-bit signed integer.

```mlang
int x = 42;
int neg = -7;
```

Arithmetic: `+` `-` `*` `/` (integer division, truncates toward zero).

### `dec`

64-bit IEEE 754 floating-point.

```mlang
dec pi = 3.14159;
dec ratio = 1.0 / 3.0;
```

Mixed `int`/`dec` arithmetic promotes to `dec`:

```mlang
dec result = 2 + 0.5;   # 2.5
```

### `txt`

UTF-8 string.

```mlang
txt greeting = "Hello, world!";
```

Concatenation with `+`:

```mlang
txt full = "Hello, " + name;
txt mixed = "Score: " + 100;   # coerces int to txt
```

### `bool`

`true` or `false`.

```mlang
bool active = true;
bool done   = false;
```

Logical operators: `&&` `||` `!`

## Composite types

### `array<T>`

Ordered, homogeneous, dynamically sized.

```mlang
array<int> scores = [10, 20, 30];
array<txt> tags   = ["rust", "lang"];
```

**Methods:**

| Method                | Returns | Description                     |
|-----------------------|---------|---------------------------------|
| `a.len()`             | `int`   | Number of elements              |
| `a.push(v)`           | `int`   | Appends value, returns new len  |
| `a.pop()`             | `T`     | Removes and returns last element|
| `a.clear()`           | `int`   | Empties array, returns 0        |
| `a.contains(v)`       | `bool`  | Membership test                 |
| `a.slice(start, end)` | `array<T>` | Sub-array (exclusive end)    |

Index access and assignment:

```mlang
print(scores[0]);    # 10
scores[1] = 99;
```

### `struct`

Named collection of typed fields. See [Structs](./structs) for full details.

```mlang
struct Point { int x; int y }
let p = Point { x = 3, y = 7 };
print(p.x);
```

## Type conversions

Explicit casts using built-in functions:

```mlang
int n   = int("42");      # parse txt → int
dec d   = dec(n);         # widen int → dec
int i   = int(3.9);       # truncate dec → int  (= 3)
txt s   = txt(100);       # format int → txt
```

`read()` always returns `txt`; use `int()` / `dec()` to parse user input:

```mlang
int age = int(read("Age: "));
```
