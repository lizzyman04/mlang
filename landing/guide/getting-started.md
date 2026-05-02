# Getting Started

MLang is a statically typed, interpreted scripting language. Every program begins in a `main()` function. The compiler is a single Rust binary — no VM, no runtime to install.

## Prerequisites

- [Rust](https://rustup.rs) 1.80 or newer
- `cargo` (ships with Rust)

## Build from source

```bash
git clone https://github.com/lizzyman04/mlang.git
cd mlang
cargo build --release
```

The binary lands at `target/release/mlang`.

## Your first program

Create `hello.mth`:

```mlang
main() {
    print("Hello, MLang!");
}
```

Run it:

```bash
cargo run -- hello.mth
# or, after building:
./target/release/mlang hello.mth
```

Output:

```
Hello, MLang!
```

## Variables

MLang is statically typed. Declare with an explicit type or use `let` for inference:

```mlang
main() {
    int age = 25;
    dec pi  = 3.14159;
    txt msg = "ready";
    bool ok = true;

    let x = 42;        # inferred as int
    let s = "hello";   # inferred as txt

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
