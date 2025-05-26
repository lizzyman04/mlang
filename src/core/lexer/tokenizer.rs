use crate::core::lexer::token::{Token, TokenKind};

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;
    let mut column = 0;

    while let Some(&ch) = chars.peek() {
        column += 1;

        match ch {
            c if c.is_whitespace() => {
                if c == '\n' {
                    line += 1;
                    column = 0;
                }
                chars.next();
            }

            c if c.is_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let token_kind = match ident.as_str() {
                    "main" | "print" => TokenKind::Keyword(ident),
                    _ => TokenKind::Identifier(ident),
                };

                tokens.push(Token { kind: token_kind, line, column });
            }

            c if c.is_digit(10) => {
                let mut num = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) {
                        num.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    kind: TokenKind::Int(num.parse().unwrap()),
                    line,
                    column,
                });
            }

            '"' => {
                chars.next(); // consume "
                let mut txt = String::new();
                while let Some(ch) = chars.next() {
                    if ch == '"' {
                        break;
                    }
                    txt.push(ch);
                }

                tokens.push(Token {
                    kind: TokenKind::Txt(txt),
                    line,
                    column,
                });
            }

            '(' => { chars.next(); tokens.push(Token { kind: TokenKind::LeftParen, line, column }); }
            ')' => { chars.next(); tokens.push(Token { kind: TokenKind::RightParen, line, column }); }
            '{' => { chars.next(); tokens.push(Token { kind: TokenKind::LeftBrace, line, column }); }
            '}' => { chars.next(); tokens.push(Token { kind: TokenKind::RightBrace, line, column }); }
            ';' => { chars.next(); tokens.push(Token { kind: TokenKind::Semicolon, line, column }); }
            '=' => { chars.next(); tokens.push(Token { kind: TokenKind::Equal, line, column }); }

            _ => {
                return Err(format!("Unexpected character '{}' at line {}, column {}", ch, line, column));
            }
        }
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        line,
        column,
    });

    Ok(tokens)
}
