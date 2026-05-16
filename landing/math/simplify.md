# Simplifying Expressions

## `math.simplify(expression: txt) -> txt`

Reduces an expression to its simplest form by combining like terms and folding constants. Returns the simplified expression as a string.

```mlang
result = math.simplify("2x + x + 4 - 1")
print(result)   # 3x + 3
```

## Examples

### Combining like terms

```mlang
main() {
    print(math.simplify("2x + x + 4 - 1"))   # 3x + 3
    print(math.simplify("3x - x + 5 + 2"))   # 2x + 7
}
```

### Constant folding

```mlang
main() {
    print(math.simplify("4 + 3"))   # 7
    print(math.simplify("10 - 6"))  # 4
}
```
