#[derive(Debug, Clone, PartialEq)]
pub enum SimpleSymbolKind {
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

impl SimpleSymbolKind {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '(' => Some(SimpleSymbolKind::LeftParen),
            ')' => Some(SimpleSymbolKind::RightParen),
            '{' => Some(SimpleSymbolKind::LeftBrace),
            '}' => Some(SimpleSymbolKind::RightBrace),
            ';' => Some(SimpleSymbolKind::Semicolon),
            ',' => Some(SimpleSymbolKind::Comma),
            '+' => Some(SimpleSymbolKind::Plus),
            '-' => Some(SimpleSymbolKind::Minus),
            '*' => Some(SimpleSymbolKind::Star),
            '/' => Some(SimpleSymbolKind::Slash),
            '=' => Some(SimpleSymbolKind::Equal),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            SimpleSymbolKind::LeftParen => '(',
            SimpleSymbolKind::RightParen => ')',
            SimpleSymbolKind::LeftBrace => '{',
            SimpleSymbolKind::RightBrace => '}',
            SimpleSymbolKind::Semicolon => ';',
            SimpleSymbolKind::Comma => ',',
            SimpleSymbolKind::Plus => '+',
            SimpleSymbolKind::Minus => '-',
            SimpleSymbolKind::Star => '*',
            SimpleSymbolKind::Slash => '/',
            SimpleSymbolKind::Equal => '=',
            SimpleSymbolKind::Eof => ' ',
        }
    }
}