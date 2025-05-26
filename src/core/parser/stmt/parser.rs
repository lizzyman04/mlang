use crate::core::parser::ast::ASTNode;
use crate::core::parser::parser::Parser;
use crate::core::lexer::token::TokenKind;

use super::function::parse_function_decl;
use super::print::parse_print_stmt;
use super::var::parse_var_decl;

pub fn parse_statement(parser: &mut Parser) -> Result<ASTNode, String> {
    if let Some(token) = parser.peek() {
        match &token.kind {
            TokenKind::Keyword(kw) if kw == "main" => parse_function_decl(parser),
            TokenKind::Keyword(kw) if kw == "print" => parse_print_stmt(parser),
            TokenKind::Keyword(kw) if ["int", "dec", "txt"].contains(&kw.as_str()) => {
                parse_var_decl(parser)
            }
            _ => Err(format!(
                "Unexpected token {:?} at line {}, column {}",
                token.kind, token.line, token.column
            )),
        }
    } else {
        Err("Unexpected end of input".to_string())
    }
}
