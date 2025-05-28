use crate::core::lexer::token::{Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub fn read_string(
    chars: &mut Peekable<Chars>,
    line: usize,
    column: usize,
) -> Result<Token, String> {
    chars.next();
    let mut txt = String::new();

    while let Some(ch) = chars.next() {
        if ch == '"' {
            break;
        }
        txt.push(ch);
    }

    Ok(Token {
        kind: TokenKind::Txt(txt),
        line,
        column,
    })
}
