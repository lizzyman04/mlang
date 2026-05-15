use std::iter::Peekable;
use std::str::Chars;
use super::token::MathToken;

pub fn parse_number(chars: &mut Peekable<Chars>) -> Result<MathToken, String> {
    let mut buf = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            buf.push(c);
            chars.next();
        } else {
            break;
        }
    }

    buf.parse::<f64>()
        .map(MathToken::Num)
        .map_err(|_| format!("invalid number: '{}'", buf))
}
