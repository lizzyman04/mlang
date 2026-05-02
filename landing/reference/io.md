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

### `read()` → `txt`

Reads a line from stdin, trims whitespace, returns as `txt`.

```mlang
txt name = read();
```

### `read(txt prompt)` → `txt`

Prints the prompt (no newline) then reads a line.

```mlang
txt name = read("Enter your name: ");
```

### `read_int()` / `read_int(txt prompt)` → `int`

Reads and parses input as `int`. Raises a runtime error if the input is not a valid integer.

```mlang
int n = read_int("Enter a number: ");
```

### `read_dec()` / `read_dec(txt prompt)` → `dec`

Reads and parses input as `dec`. Raises a runtime error if the input is not a valid decimal.

```mlang
dec x = read_dec("Enter a decimal: ");
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
    txt name = read("What is your name? ");
    print("Hello, " + name);

    int age = int(read("Your age: "));
    print("Next year you will be " + (age + 1));

    print("Press Enter to continue...");
    read();
    print("Done!");
}
```
