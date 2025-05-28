#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Int(i64),
    Dec(f64),
    Txt(String),
    Bool(bool),
    Keyword(String),
    Identifier(String),

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl TokenKind {
    pub fn display(&self) -> String {
        match self {
            TokenKind::Equal => "'='".to_string(),
            TokenKind::LeftParen => "'('".to_string(),
            TokenKind::RightParen => "')'".to_string(),
            TokenKind::Semicolon => "';'".to_string(),
            TokenKind::Comma => "','".to_string(),
            TokenKind::Keyword(k) => format!("keyword '{}'", k),
            TokenKind::Identifier(s) => format!("identifier '{}'", s),
            TokenKind::Int(i) => format!("integer '{}'", i),
            TokenKind::Txt(t) => format!("string \"{}\"", t),
            TokenKind::Bool(b) => format!("boolean '{}'", b),
            other => format!("{:?}", other),
        }
    }
}