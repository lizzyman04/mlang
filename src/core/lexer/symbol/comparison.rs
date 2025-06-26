#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonSymbolKind {
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

impl ComparisonSymbolKind {
    pub fn from_pair(first: char, second: Option<char>) -> Option<Self> {
        match (first, second) {
            ('=', Some('=')) => Some(Self::EqualEqual),
            ('!', Some('=')) => Some(Self::NotEqual),
            ('<', Some('=')) => Some(Self::LessEqual),
            ('>', Some('=')) => Some(Self::GreaterEqual),
            ('<', _) => Some(Self::Less),
            ('>', _) => Some(Self::Greater),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::EqualEqual => "==",
            Self::NotEqual => "!=",
            Self::Less => "<",
            Self::Greater => ">",
            Self::LessEqual => "<=",
            Self::GreaterEqual => ">=",
        }
    }
}
