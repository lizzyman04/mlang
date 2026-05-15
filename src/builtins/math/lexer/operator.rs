use std::iter::Peekable;
use std::str::Chars;
use super::token::MathToken;

pub fn parse_operator(ch: char, _chars: &mut Peekable<Chars>) -> Option<MathToken> {
    match ch {
        '+' => Some(MathToken::Plus),
        '-' => Some(MathToken::Minus),
        '*' => Some(MathToken::Star),
        '/' => Some(MathToken::Slash),
        '^' => Some(MathToken::Caret),
        '=' => Some(MathToken::Eq),
        '(' => Some(MathToken::LParen),
        ')' => Some(MathToken::RParen),
        _ => None,
    }
}
