# Getting Started

MLang is a statically typed, interpreted scripting language. Every program begins in a `main()` function.

## Install

```bash
curl -fsSL https://lizzyman04.github.io/mlang/install.sh | sh
```

## Your first program

Create `hello.mth`:

```mlang
main() {
    print("Hello, MLang!");
}
```

Run it:

```bash
mlang hello.mth
```

Output:

```
Hello, MLang!
```

## Variables

MLang is statically typed. Declare variables with an explicit type:

```mlang
main() {
    int age = 25;
    dec pi  = 3.14159;
    txt msg = "ready";
    bool ok = true;

    print(age + 1);
    print("Message: " + msg);
}
```

## Control flow

```mlang
main() {
    int n = 7;

    if n > 5 {
        print("big");
    } else {
        print("small");
    }

    for i in 0..5 {
        print(i);
    }
}
```

## Next steps

- [Installation options](./installation) — add `mlang` to your `PATH`
- [Examples](./examples) — more complete programs
- [Language reference](../reference/syntax) — full syntax guide
