use crate::core::lexer::symbol::math::MathSymbolKind;
use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::{Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub fn read_symbol(
    chars: &mut Peekable<Chars>,
    line: usize,
    column: usize,
) -> Result<Option<Token>, String> {
    let symbol = chars.next().unwrap();

    let kind = if let Some(simple) = SimpleSymbolKind::from_char(symbol) {
        TokenKind::SimpleSymbol(simple)
    } else if let Some(math) = MathSymbolKind::from_char(symbol) {
        TokenKind::MathSymbol(math)
    } else {
        return Ok(None);
    };

    Ok(Some(Token { kind, line, column }))
}
