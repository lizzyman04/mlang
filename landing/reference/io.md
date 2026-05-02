# I/O

## Output

### `print(value)`

Writes a value to stdout followed by a newline. Accepts any type.

```mlang
print("Hello");       # Hello
print(42);            # 42
print(3.14);          # 3.14
print(true);          # true
print([1, 2, 3]);     # [1, 2, 3]
```

String coercion: when the left operand of `+` is `txt`, the right operand is automatically converted:

```mlang
print("Score: " + 100);     # Score: 100
print("Pi ≈ " + 3.14);     # Pi ≈ 3.14
print("Done: " + true);     # Done: true
```

## Input

### `read()` / `read(txt prompt)` → `int` | `dec` | `txt`

Reads a line from stdin and trims whitespace. The return type is inferred automatically from the input:

| Input looks like | Return type |
|---|---|
| Empty string | `txt` |
| Integer (e.g. `42`, `-7`) | `int` |
| Decimal (e.g. `3.14`, `-0.5`) | `dec` |
| Anything else | `txt` |

```mlang
let a = read();                     # "42"    → int
let b = read();                     # "3.14"  → dec
let c = read();                     # "hello" → txt
let name = read("Your name: ");     # prompt, then auto-detect
```

Use `let` for auto-detected reads. If you need a specific type, wrap with a cast:

```mlang
txt raw   = txt(read("Value: "));   # force txt regardless of content
int exact = int(read("Number: "));  # force int, error if not parseable
```

## Type conversion

Convert between types explicitly with cast functions:

| Call       | Input types          | Output  | Notes                        |
|------------|----------------------|---------|------------------------------|
| `int(x)`   | `dec`, `txt`, `int`  | `int`   | `dec` truncates toward zero  |
| `dec(x)`   | `int`, `txt`, `dec`  | `dec`   |                              |
| `txt(x)`   | `int`, `dec`, `bool` | `txt`   | Formats the value as a string|

```mlang
int age   = int(read("Age: "));        # parse user input
dec ratio = dec(3);                    # 3.0
txt label = txt(age + 1);             # number to string
int trunc = int(9.9);                  # 9 (truncated)
```

## Full example

```mlang
main() {
    let name  = read("What is your name? ");   # txt
    let age   = read("Your age: ");            # int  (if user types a number)
    let price = read("Item price: ");          # dec  (if user types e.g. 4.99)

    print("Hello, " + name);
    print("Next year you will be " + (age + 1));
    print("Total with tax: " + (price * 1.1));
}
```
