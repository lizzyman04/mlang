#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Int(i64),
    Dec(f64),
    Txt(String),
    Identifier(String),
    Keyword(String),

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Equal,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}
