use super::symbol::{
    comparison::ComparisonSymbolKind, math::MathSymbolKind, simple::SimpleSymbolKind,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Int(i64),
    Dec(f64),
    Txt(String),
    Bool(bool),
    Keyword(String),
    Identifier(String),

    MathSymbol(MathSymbolKind),
    SimpleSymbol(SimpleSymbolKind),
    ComparisonSymbol(ComparisonSymbolKind),
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
            TokenKind::Keyword(k) => format!("keyword '{}'", k),
            TokenKind::Identifier(s) => format!("identifier '{}'", s),
            TokenKind::Int(i) => format!("integer '{}'", i),
            TokenKind::Dec(f) => format!("decimal '{}'", f),
            TokenKind::Txt(t) => format!("string \"{}\"", t),
            TokenKind::Bool(b) => format!("boolean '{}'", b),
            TokenKind::MathSymbol(sym) => format!("'{}'", sym.to_char()),
            TokenKind::SimpleSymbol(sym) => format!("'{}'", sym.to_char()),
            TokenKind::ComparisonSymbol(sym) => format!("'{}'", sym.to_str()),
            // other => format!("{:?}", other),
        }
    }
}
