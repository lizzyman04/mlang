# MLang Grammar Specification

This document defines the complete grammar and syntax rules of the MLang programming language. MLang is a statically typed, block-structured language with minimal keywords and built-in support for mathematical expression solving.

The grammar follows a high-level EBNF-inspired description for clarity and implementation guidance.

---

## 🧾 File Structure

```mlang
main() {
    // your code starts here
}
````

Every valid MLang file must contain a `main()` function. Execution begins there.

Other functions can be declared before or after `main()` and will be resolved in the symbol table.

---

## 📦 Variable Declarations

### Syntax

```mlang
<Type> <identifier> = <expression>;
```

### Example

```mlang
int x = 5;
dec pi = 3.14159;
txt greeting = "Hello, world!";
```

### Types

* `int`: Integer (whole numbers)
* `dec`: Decimal / floating point (high precision)
* `txt`: String literal (renamed `string`)

---

## 🧠 Expressions

Expressions in MLang can be:

* Literals: `42`, `3.14`, `"text"`
* Variables: `x`, `pi`
* Operations: `+`, `-`, `*`, `/`, `%`, `^`
* Grouped: `(a + b) * c`

### Examples

```mlang
x = 2 + 3 * 4;
area = pi * r ^ 2;
message = "Hello, " + name;
```

> Operator precedence is respected:
> `^` > `*` `/` `%` > `+` `-`

---

## 🧱 Statements

* **Variable assignment**
* **Function call**
* **Loop**
* **Return**
* **Console output**

Statements are terminated by semicolons `;`.

```mlang
x = 5;
print("The result is " + x);
```

---

## 🔁 Loops (For)

### Syntax

```mlang
for <Type> <var> = <start>; <condition>; <update> {
    <block>
}
```

### Example

```mlang
for int i = 0; i < 10; i = i + 1 {
    print(i);
}
```

Future support: `while`, `do-while`, `foreach`

---

## 🔧 Functions

### Syntax

```mlang
<function_name>(<Type1> <arg1>, <Type2> <arg2>, ...) {
    <statements>
    return <expression>; // optional
}
```

### Example

```mlang
add(int a, int b) {
    return a + b;
}
```

### Return Type

Functions currently assume implicit return types. Future plans include explicit return typing:

```mlang
int add(int a, int b) { ... }
```

---

## 🔍 Math Engine Expressions

Math expressions are declared as strings and passed to the `math.solve()` function.

```mlang
expr = math.solve("2x + 4 = 10");
```

### Post-evaluation API

```mlang
print(expr.result());   // → 3
print(expr.step(1));    // → "2x = 6"
print(expr.steps());    // → ["2x + 4 = 10", "2x = 6", "x = 3"]
```

Supported syntax inside math strings includes:

* Variables: `x`, `y`, `z`
* Powers: `^`
* Equality: `=`
* Grouping: `()`

Future expansion: `integrate`, `derive`, `simplify`, `plot`, etc.

---

## 📤 Console Output

```mlang
print("message");    // Output to stdout
error("msg");      // Output to stderr
```

Use `+` for string concatenation and variables:

```mlang
print("Hello, " + name);
```

---

## ⛔ Reserved Words

```
int, dec, txt, bool, for, return, main, print, error, math, solve, step, steps, result
```

> User-defined functions and variables must not use these reserved keywords.

---

## 📃 Comments

Single-line comments use `//`. Multi-line comments are **not supported yet**.

```mlang
// This is a comment
int x = 10; // Set x to ten
```

---

## 🚀 Sample Program

```mlang
main() {
    int a = 2;
    int b = 3;
    int sum = a + b;

    print("Sum is " + sum);

    expr = math.solve("2x + 4 = 10");
    print("x = " + expr.result());
}
```

---

## ✅ Grammar Summary (Simplified)

```ebnf
program         ::= { function_decl } main_decl
main_decl       ::= "main()" block

function_decl   ::= identifier "(" [params] ")" block
params          ::= param { "," param }
param           ::= type identifier

block           ::= "{" { statement } "}"
statement       ::= var_decl | assign_stmt | expr_stmt | loop_stmt | return_stmt | print_stmt
var_decl        ::= type identifier "=" expression ";"
assign_stmt     ::= identifier "=" expression ";"
expr_stmt       ::= expression ";"
return_stmt     ::= "return" expression ";"
print_stmt      ::= "print(" expression ");"

loop_stmt       ::= "for" type identifier "=" expression ";" expression ";" assign_stmt block

expression      ::= term { ("+" | "-") term }
term            ::= factor { ("*" | "/" | "%") factor }
factor          ::= primary { "^" primary }
primary         ::= number | string | identifier | "(" expression ")" | function_call
function_call   ::= identifier "(" [args] ")"
args            ::= expression { "," expression }

type            ::= "int" | "dec" | "txt" | "bool"
identifier      ::= letter { letter | digit | "_" }
number          ::= digit { digit } [ "." digit { digit } ]
string          ::= "\"" { char } "\""
```

---

> 🔗 For internals of the math engine, refer to [engine.md](engine.md)