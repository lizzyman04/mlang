#[derive(Debug, Clone, PartialEq)]
pub enum SimpleSymbolKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Not,
    And,
    Or,
    DotDot,
    Eof,
}

impl SimpleSymbolKind {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '(' => Some(SimpleSymbolKind::LeftParen),
            ')' => Some(SimpleSymbolKind::RightParen),
            '{' => Some(SimpleSymbolKind::LeftBrace),
            '}' => Some(SimpleSymbolKind::RightBrace),
            '[' => Some(SimpleSymbolKind::LeftBracket),
            ']' => Some(SimpleSymbolKind::RightBracket),
            ';' => Some(SimpleSymbolKind::Semicolon),
            ',' => Some(SimpleSymbolKind::Comma),
            '.' => Some(SimpleSymbolKind::Dot),
            '+' => Some(SimpleSymbolKind::Plus),
            '-' => Some(SimpleSymbolKind::Minus),
            '*' => Some(SimpleSymbolKind::Star),
            '/' => Some(SimpleSymbolKind::Slash),
            '=' => Some(SimpleSymbolKind::Equal),
            '!' => Some(SimpleSymbolKind::Not),
            _ => None,
        }
    }

    pub fn to_display(&self) -> &'static str {
        match self {
            SimpleSymbolKind::LeftParen => "(",
            SimpleSymbolKind::RightParen => ")",
            SimpleSymbolKind::LeftBrace => "{",
            SimpleSymbolKind::RightBrace => "}",
            SimpleSymbolKind::LeftBracket => "[",
            SimpleSymbolKind::RightBracket => "]",
            SimpleSymbolKind::Semicolon => ";",
            SimpleSymbolKind::Comma => ",",
            SimpleSymbolKind::Dot => ".",
            SimpleSymbolKind::Plus => "+",
            SimpleSymbolKind::Minus => "-",
            SimpleSymbolKind::Star => "*",
            SimpleSymbolKind::Slash => "/",
            SimpleSymbolKind::Equal => "=",
            SimpleSymbolKind::Not => "!",
            SimpleSymbolKind::And => "&&",
            SimpleSymbolKind::Or => "||",
            SimpleSymbolKind::DotDot => "..",
            SimpleSymbolKind::Eof => "EOF",
        }
    }

    pub fn to_char(&self) -> char {
        self.to_display().chars().next().unwrap_or('\0')
    }
}
