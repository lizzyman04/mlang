use crate::core::lexer::token::{Token, TokenKind};

use super::identifiers::read_identifier_or_keyword;
use super::numbers::read_number;
use super::strings::read_string;
use super::symbols::read_symbol;

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
                tokens.push(read_number(&mut chars, line, column)?);
            }
            c if c.is_alphabetic() || c == '_' => {
                tokens.push(read_identifier_or_keyword(&mut chars, line, column)?);
            }
            '"' => {
                tokens.push(read_string(&mut chars, line, column)?);
            }
            _ => {
                if let Some(tok) = read_symbol(&mut chars, line, column)? {
                    tokens.push(tok);
                } else {
                    return Err(format!(
                        "Unexpected character '{}' at line {}, column {}",
                        ch, line, column
                    ));
                }
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
