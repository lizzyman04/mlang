use std::iter::Peekable;
use std::str::Chars;
use super::token::MathToken;

const FUNCTIONS: &[&str] = &["sqrt", "sin", "cos", "tan", "log", "exp", "abs", "pow"];

pub fn parse_identifier(chars: &mut Peekable<Chars>) -> Result<MathToken, String> {
    let mut buf = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            buf.push(c);
            chars.next();
        } else {
            break;
        }
    }

    let token = match buf.as_str() {
        "pi" => MathToken::Num(std::f64::consts::PI),
        "e" => MathToken::Num(std::f64::consts::E),
        "tau" => MathToken::Num(std::f64::consts::TAU),
        "inf" => MathToken::Num(f64::INFINITY),
        name if FUNCTIONS.contains(&name) => MathToken::Func(buf),
        _ => MathToken::Ident(buf),
    };

    Ok(token)
}
