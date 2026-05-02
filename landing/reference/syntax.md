# Syntax Overview

## Program structure

Every MLang file must contain a `main()` function. Functions and struct declarations appear at the top level, outside `main`.

```mlang
# top-level declarations
struct Color { int r; int g; int b }

int square(int n) {
    return n * n;
}

# entry point
main() {
    print(square(7));
}
```

## Comments

```mlang
# This is a single-line comment
```

## Statements

Every statement ends with a semicolon.

```mlang
int x = 10;
print(x);
x = x + 1;
```

## Variable declarations

```mlang
int   count  = 0;
dec   ratio  = 0.5;
txt   label  = "hello";
bool  active = true;

# Type inference
let n = 42;       # int
let s = "world";  # txt
```

## Assignment

```mlang
x = x + 1;
arr[2] = 99;
point.x = 5;
```

## Operators

| Category    | Operators                         |
|-------------|-----------------------------------|
| Arithmetic  | `+` `-` `*` `/`                   |
| Comparison  | `==` `!=` `<` `>` `<=` `>=`      |
| Logical     | `&&` `\|\|` `!`                   |
| Concatenate | `+` (when left operand is `txt`)  |

## If / else

```mlang
if condition {
    # ...
} else {
    # ...
}
```

## While loop

```mlang
while condition {
    # ...
}
```

## For-range loop

```mlang
for i in 0..10 {
    print(i);   # 0 through 9
}
```

## For-in loop (arrays)

```mlang
for item in my_array {
    print(item);
}
```

## Break and continue

```mlang
while true {
    if done {
        break;
    }
    if skip {
        continue;
    }
}
```

## Return

```mlang
int double(int n) {
    return n * 2;
}
```
