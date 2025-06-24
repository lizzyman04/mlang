# 🖥️ MLang GUI

MLang now includes a built-in graphical IDE, designed to simplify the process of writing and running math expressions interactively.

---

## ✨ Features

- **Syntax-highlighted code editor**
- **Run button** to evaluate expressions instantly
- **Tabbed output** for `Output` and `Errors`
- **Math symbol panel** for quick access to `π`, `√`, `∑`, etc.
- **Documentation tab** with inline guides and tips

---

## 🏁 How to Launch

### Run the GUI:

```bash
cargo run
````

This will open a windowed desktop interface.

### Run a file in terminal mode:

```bash
cargo run -- --cli examples/hello.mlang
```

---

## 🛠️ Planned Extensions

* File open/save support
* Live diagnostics (underlining errors as you type)
* Math graph plotting
* Dark mode toggle

---

## 📁 Source Layout

The GUI code is located in:

```
src/gui/
```

Organized by:

* `editor/`: Code editor component
* `tabs/`: Output and error display
* `symbols/`: Math symbol inserter
* `docs/`: Embedded documentation

---

Built with [`egui`](https://github.com/emilk/egui) and [`eframe`](https://github.com/emilk/eframe) for fast native performance.