# MLang — A Math-First Programming Language

**MLang** is a modern programming language focused on symbolic and numeric computation.  
Now with a beautiful built-in IDE for editing and running `.mlang` code interactively!

---

## 📚 Table of Contents

- [Overview](docs/overview.md)
- [Language Grammar](docs/grammar.md)
- [Runtime Architecture](docs/runtime.md)
- [Math Engine](docs/engine.md)
- [GUI Mode](docs/gui.md)
- [Examples](examples/)
- [License](LICENSE)

---

## 🚀 Getting Started

### Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
````

### Build MLang

```bash
cargo build --release
```

### Run MLang

Run from terminal:

```bash
cargo run -- --cli examples/hello.mlang
```

Or launch the GUI IDE:

```bash
cargo run
```

---

## 🧠 Contributing

* Use clear, atomic Git commits (one feature or fix per commit)
* Code should be formatted with `rustfmt`

### Example Commit Message

```
feat: add lexer module and implement token scanning
```

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.