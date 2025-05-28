use crate::core::lexer::token::{Token, TokenKind};

use super::rules;

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

            c if c.is_ascii_digit() => {
                let mut number = String::new();
                let mut has_dot = false;

                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        number.push(chars.next().unwrap());
                    } else if next_ch == '.' && !has_dot {
                        has_dot = true;
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                let token_kind = if has_dot {
                    match number.parse::<f64>() {
                        Ok(f) => TokenKind::Dec(f),
                        Err(_) => {
                            return Err(format!(
                                "Invalid decimal '{}' at line {}, column {}",
                                number, line, column
                            ));
                        }
                    }
                } else {
                    match number.parse::<i64>() {
                        Ok(i) => TokenKind::Int(i),
                        Err(_) => {
                            return Err(format!(
                                "Invalid integer '{}' at line {}, column {}",
                                number, line, column
                            ));
                        }
                    }
                };

                tokens.push(Token {
                    kind: token_kind,
                    line,
                    column,
                });
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

                let token_kind = if rules::is_keyword(&ident) {
                    TokenKind::Keyword(ident)
                } else {
                    TokenKind::Identifier(ident)
                };

                tokens.push(Token {
                    kind: token_kind,
                    line,
                    column,
                });
            }

            '"' => {
                chars.next();
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

            '(' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::LeftParen,
                    line,
                    column,
                });
            }
            ')' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::RightParen,
                    line,
                    column,
                });
            }
            '{' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::LeftBrace,
                    line,
                    column,
                });
            }
            '}' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::RightBrace,
                    line,
                    column,
                });
            }
            ';' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::Semicolon,
                    line,
                    column,
                });
            }
            '=' => {
                chars.next();
                tokens.push(Token {
                    kind: TokenKind::Equal,
                    line,
                    column,
                });
            }

            _ => {
                return Err(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    ch, line, column
                ));
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
