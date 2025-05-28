use crate::core::lexer::token::{Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub fn read_symbol(
    chars: &mut Peekable<Chars>,
    line: usize,
    column: usize,
) -> Result<Option<Token>, String> {
    let symbol = chars.next().unwrap();
    let kind = match symbol {
        '(' => TokenKind::LeftParen,
        ')' => TokenKind::RightParen,
        '{' => TokenKind::LeftBrace,
        '}' => TokenKind::RightBrace,
        ';' => TokenKind::Semicolon,
        '=' => TokenKind::Equal,
        '+' => TokenKind::Plus,
        '-' => TokenKind::Minus,
        '*' => TokenKind::Star,
        '/' => TokenKind::Slash,
        ',' => TokenKind::Comma,
        _ => return Ok(None),
    };

    Ok(Some(Token { kind, line, column }))
}
