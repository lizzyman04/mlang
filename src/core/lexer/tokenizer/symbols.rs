use crate::core::lexer::symbol::comparison::ComparisonSymbolKind;
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
    let first = chars.next().unwrap();
    let second = chars.peek().copied();

    if let Some(comp) = ComparisonSymbolKind::from_pair(first, second) {
        if matches!(first, '=' | '!' | '<' | '>') && second == Some('=') {
            chars.next();
        }

        return Ok(Some(Token {
            kind: TokenKind::ComparisonSymbol(comp),
            line,
            column,
        }));
    }

    let kind = if let Some(simple) = SimpleSymbolKind::from_char(first) {
        TokenKind::SimpleSymbol(simple)
    } else if let Some(math) = MathSymbolKind::from_char(first) {
        TokenKind::MathSymbol(math)
    } else {
        return Ok(None);
    };

    Ok(Some(Token { kind, line, column }))
}
