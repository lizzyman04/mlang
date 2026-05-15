#[derive(Debug, Clone, PartialEq)]
pub enum MathToken {
    Num(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Eq,
    LParen,
    RParen,
    Func(String),
}

impl MathToken {
    pub fn display(&self) -> String {
        match self {
            MathToken::Num(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            MathToken::Ident(s) => s.clone(),
            MathToken::Func(s) => s.clone(),
            MathToken::Plus => "+".to_string(),
            MathToken::Minus => "-".to_string(),
            MathToken::Star => "*".to_string(),
            MathToken::Slash => "/".to_string(),
            MathToken::Caret => "^".to_string(),
            MathToken::Eq => "=".to_string(),
            MathToken::LParen => "(".to_string(),
            MathToken::RParen => ")".to_string(),
        }
    }
}
