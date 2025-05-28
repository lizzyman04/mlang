use crate::core::lexer::rules::is_variable_type;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::parser::Parser;

use super::decider::parse_var_or_function_decl;
use super::func::parse_function_decl;
use super::print::parse_print_stmt;
use super::r#return::parse_return_stmt;

pub fn parse_statement(parser: &mut Parser) -> Result<ASTNode, String> {
    if let Some(token) = parser.peek() {
        match &token.kind {
            TokenKind::Keyword(kw) if kw == "main" => parse_function_decl(parser),
            TokenKind::Keyword(kw) if kw == "print" => parse_print_stmt(parser),
            TokenKind::Keyword(kw) if kw == "return" => parse_return_stmt(parser),
            TokenKind::Keyword(kw) if is_variable_type(kw) => {
                let var_type = kw.clone();
                parser.advance();
                parse_var_or_function_decl(parser, &var_type)
            }
            _ => Err(format!(
                "Unexpected token {:?} at line {}, column {}",
                token.kind.display(), token.line, token.column
            )),
        }
    } else {
        Err("Unexpected end of input".to_string())
    }
}
