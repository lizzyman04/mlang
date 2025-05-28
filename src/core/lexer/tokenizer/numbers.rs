use crate::core::lexer::token::{Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub fn read_number(
    chars: &mut Peekable<Chars>,
    line: usize,
    column: usize,
) -> Result<Token, String> {
    let mut number = String::new();
    let mut has_dot = false;

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            number.push(ch);
            chars.next();
        } else if ch == '.' && !has_dot {
            has_dot = true;
            number.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    let kind = if has_dot {
        match number.parse::<f64>() {
            Ok(f) => TokenKind::Dec(f),
            Err(_) => return Err(format!("Invalid decimal '{}'", number)),
        }
    } else {
        match number.parse::<i64>() {
            Ok(i) => TokenKind::Int(i),
            Err(_) => return Err(format!("Invalid integer '{}'", number)),
        }
    };

    Ok(Token {
        kind,
        line,
        column,
    })
}
