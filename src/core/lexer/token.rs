use super::symbol::{math::MathSymbolKind, simple::SimpleSymbolKind};

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
            TokenKind::SimpleSymbol(sym) => format!("'{}'", sym.to_char()),
            TokenKind::MathSymbol(sym) => format!("'{}'", sym.to_char()),
            // other => format!("{:?}", other),
        }
    }
}
