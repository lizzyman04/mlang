# MLang

[![CI](https://github.com/lizzyman04/mlang/actions/workflows/ci.yml/badge.svg)](https://github.com/lizzyman04/mlang/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-online-blue)](https://lizzyman04.github.io/mlang)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)

A math-first, statically typed programming language implemented as a tree-walking interpreter in Rust.

**[Documentation](https://lizzyman04.github.io/mlang)** · [Examples](examples/) · [Contributing](docs/README.md)

---

## Install

```bash
curl -fsSL https://lizzyman04.github.io/mlang/install.sh | sh
```

Or build from source:

```bash
git clone https://github.com/lizzyman04/mlang.git
cd mlang
cargo build --release
./target/release/mlang examples/hello.mth
```

## Example

```mlang
struct Point {
    int x
    int y
}

int distance(Point a, Point b) {
    int dx = a.x - b.x;
    int dy = a.y - b.y;
    return dx * dx + dy * dy;
}

main() {
    Point p1 = Point{ x = 0, y = 0 };
    Point p2 = Point{ x = 3, y = 4 };
    print(distance(p1, p2));   # 25
}
```

## Language features

- Types: `int`, `dec`, `txt`, `bool`, `array<T>`, structs
- Control flow: `if`/`else`, `while`, `for`/`in`, `break`, `continue`
- Functions with recursion and forward references
- User-defined structs with field access and mutation
- Built-in I/O: `print`, `read`, `read_int`, `read_dec`
- Type casts: `int(x)`, `dec(x)`, `txt(x)`

## Documentation

Full language reference, contributor guide, and architecture docs are at **[lizzyman04.github.io/mlang](https://lizzyman04.github.io/mlang)**.

For internals and contribution workflow, see the [`docs/`](docs/README.md) directory.

## License

MIT — see [LICENSE](LICENSE).
