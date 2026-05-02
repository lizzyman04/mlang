# Lexer Internals

Source: `src/core/lexer/`

## Entry point

`mod.rs::tokenize(source: &str) -> Result<Vec<Token>, String>`

Iterates over the source string character-by-character using a `Peekable<Chars>` iterator. Each iteration dispatches to a specialised sub-tokenizer based on the next character.

## Token structure

```rust
pub struct Token {
    pub kind:   TokenKind,
    pub line:   usize,
    pub column: usize,
}
```

## `TokenKind` variants

```rust
pub enum TokenKind {
    // Literals
    Int(i64),
    Dec(f64),
    Txt(String),
    Bool(bool),

    // Identifiers and keywords
    Identifier(String),
    Keyword(String),

    // Symbols
    SimpleSymbol(SimpleSymbolKind),
    ComparisonSymbol(ComparisonSymbolKind),
    MathSymbol(MathSymbolKind),

    Eof,
}
```

## Keywords

Defined in `rules.rs::is_keyword()`:

```
int  dec  txt  bool  void  true  false
main  print  return  if  else
array  let  while  for  in  break  continue  struct
```

`true` and `false` are tokenised directly as `Bool(true)` / `Bool(false)` by `identifiers.rs` before the keyword check.

## Dispatch table

| First char | Sub-tokenizer | Produces |
|---|---|---|
| letter or `_` | `identifiers.rs` | `Keyword` or `Identifier` |
| digit | `numbers.rs` | `Int` or `Dec` |
| `"` | `strings.rs` | `Txt` |
| `#` | inline in dispatcher | skips to end of line |
| `<`, `>`, `=`, `!` | `symbols.rs` | `ComparisonSymbol` or `SimpleSymbol` |
| `+`, `-`, `*`, `/`, etc. | `symbols.rs` | `SimpleSymbol` or `MathSymbol` |
| whitespace | inline in dispatcher | skipped |

## Number tokenization

`numbers.rs` reads consecutive digits. If a `.` is found and followed by another digit, the token becomes `Dec`; otherwise `Int`.

## String tokenization

`strings.rs` consumes characters until the closing `"`. Escape sequences are not currently supported.

## Symbol tokenization

`symbols.rs` handles all operators. Multi-character operators (`==`, `!=`, `<=`, `>=`) are recognised by peeking one character ahead.

## Error handling

Unrecognised characters produce `Err(format!("Unexpected character '{}' at line {}, column {}", ch, line, col))`.
