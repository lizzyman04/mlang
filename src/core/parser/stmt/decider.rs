use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::parser::Parser;
use super::func::{parse_full_type, parse_function_decl_with_signature};
use super::var::parse_var_decl_with_name_and_type;

pub fn parse_var_or_function_decl(parser: &mut Parser, base_type: &str) -> Result<ASTNode, String> {
    let var_type = parse_full_type(parser, base_type)?;

    let name = {
        let tok = parser.consume_identifier()?;
        match &tok.kind {
            TokenKind::Identifier(n) => n.clone(),
            _ => unreachable!(),
        }
    };

    match parser.peek().map(|t| &t.kind) {
        Some(TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen)) => {
            parse_function_decl_with_signature(parser, var_type, &name)
        }
        Some(TokenKind::SimpleSymbol(SimpleSymbolKind::Equal)) => {
            parse_var_decl_with_name_and_type(parser, var_type, &name)
        }
        Some(other) => Err(format!(
            "Expected '(' for function or '=' for variable, found {}",
            other.display()
        )),
        None => Err("Unexpected end of input after identifier".to_string()),
    }
}
