#[derive(Debug, Clone, PartialEq)]
pub enum MathSymbolKind {
    Caret,
    Sqrt,
    Summation,
    Pi,
    Infinity,
    Percent,
}

impl MathSymbolKind {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '^' => Some(MathSymbolKind::Caret),
            '√' => Some(MathSymbolKind::Sqrt),
            '∑' => Some(MathSymbolKind::Summation),
            'π' => Some(MathSymbolKind::Pi),
            '∞' => Some(MathSymbolKind::Infinity),
            '%' => Some(MathSymbolKind::Percent),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            MathSymbolKind::Caret => '^',
            MathSymbolKind::Sqrt => '√',
            MathSymbolKind::Summation => '∑',
            MathSymbolKind::Pi => 'π',
            MathSymbolKind::Infinity => '∞',
            MathSymbolKind::Percent => '%',
        }
    }
}
