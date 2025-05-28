use crate::core::lexer::token::{Token, TokenKind};
use crate::core::lexer::rules::is_keyword;
use std::iter::Peekable;
use std::str::Chars;

pub fn read_identifier_or_keyword(
    chars: &mut Peekable<Chars>,
    line: usize,
    column: usize,
) -> Result<Token, String> {
    let mut ident = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            ident.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    let kind = match ident.as_str() {
        "true" => TokenKind::Bool(true),
        "false" => TokenKind::Bool(false),
        _ => {
            if is_keyword(&ident) {
                TokenKind::Keyword(ident)
            } else {
                TokenKind::Identifier(ident)
            }
        }
    };

    Ok(Token {
        kind,
        line,
        column,
    })
}
