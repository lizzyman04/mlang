---
layout: home

hero:
  name: MLang
  text: A Math-First Programming Language
  tagline: Statically typed, expressive, and built in Rust. Write clean scripts with native structs, functions, and I/O — no boilerplate.
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: View on GitHub
      link: https://github.com/lizzyman04/mlang

features:
  - icon: 🔢
    title: Statically Typed
    details: Every variable has an explicit type — int, dec, txt, bool, array, or struct. No runtime surprises.

  - icon: ⚡
    title: Fast & Lightweight
    details: Lexer → Parser → AST → Evaluator pipeline with zero external runtime dependencies. Starts instantly.

  - icon: 🏗️
    title: Structs & Functions
    details: First-class user-defined structs with field access and mutation. Functions support recursion and struct-typed parameters.

  - icon: 📥
    title: Unified I/O
    details: read() auto-detects int, dec, or txt from user input. Optional inline prompts. Type conversion via int(), dec(), txt() casts.

  - icon: 🔁
    title: Rich Control Flow
    details: if/else, while loops, for-range loops, for-in array loops, break, continue, and early return.

  - icon: 📦
    title: Array Builtins
    details: push, pop, clear, len, contains, slice — all as ergonomic method calls directly on array variables.
---

## A taste of MLang

```mlang
struct Point {
    int x
    int y
}

int distance_sq(Point a, Point b) {
    int dx = a.x - b.x;
    int dy = a.y - b.y;
    return dx * dx + dy * dy;
}

main() {
    let p1 = Point { x = 0, y = 0 };
    let p2 = Point { x = 3, y = 4 };
    print(distance_sq(p1, p2));   # 25

    txt name = read("Your name: ");
    print("Hello, " + name);
}
```

## Install

```bash
curl -fsSL https://lizzyman04.github.io/mlang/install.sh | sh
```