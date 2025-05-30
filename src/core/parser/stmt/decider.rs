use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::parser::Parser;
use super::func::parse_function_decl_with_signature;
use super::var::parse_var_decl_with_name_and_type;

pub fn parse_var_or_function_decl(parser: &mut Parser, var_type: &str) -> Result<ASTNode, String> {
    let name_token = parser.consume_identifier()?;
    let (name, line, column) = match &name_token.kind {
        TokenKind::Identifier(n) => (n.clone(), name_token.line, name_token.column),
        _ => {
            return Err(format!(
                "Expected identifier, found {} at line {}, column {}",
                name_token.kind.display(),
                name_token.line,
                name_token.column
            ))
        }
    };

    match parser.peek().map(|t| &t.kind) {
        Some(TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen)) => parse_function_decl_with_signature(parser, var_type, &name),
        Some(TokenKind::SimpleSymbol(SimpleSymbolKind::Equal)) => parse_var_decl_with_name_and_type(parser, var_type, &name),
        Some(other) => Err(format!(
            "Expected '(' for function or '=' for variable, found {} at line {}, column {}",
            other.display(),
            line,
            column
        )),
        None => Err("Unexpected end of input after identifier".to_string()),
    }
}

