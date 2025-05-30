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
            TokenKind::Plus => "'+'".to_string(),
            TokenKind::Minus => "'-'".to_string(),
            TokenKind::Star => "'*'".to_string(),
            TokenKind::Slash => "'/'".to_string(),
            TokenKind::LeftParen => "'('".to_string(),
            TokenKind::RightParen => "')'".to_string(),
            TokenKind::LeftBrace => "'{'".to_string(),
            TokenKind::RightBrace => "'}'".to_string(),
            TokenKind::Semicolon => "';'".to_string(),
            TokenKind::Comma => "','".to_string(),
            TokenKind::Keyword(k) => format!("keyword '{}'", k),
            TokenKind::Identifier(s) => format!("identifier '{}'", s),
            TokenKind::Int(i) => format!("integer '{}'", i),
            TokenKind::Dec(f) => format!("decimal '{}'", f),
            TokenKind::Txt(t) => format!("string \"{}\"", t),
            TokenKind::Bool(b) => format!("boolean '{}'", b),
            TokenKind::Eof => "end of file".to_string(),
        }
    }
}
