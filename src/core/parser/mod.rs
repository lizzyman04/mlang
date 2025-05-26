pub mod ast;
pub mod parser;
pub mod stmt;

use crate::core::parser::ast::ASTNode;
use crate::core::parser::parser::Parser;
use crate::core::parser::stmt::parse_statement;

pub fn parse(tokens: Vec<crate::core::lexer::token::Token>) -> Result<ASTNode, String> {
    let mut parser = Parser::new(tokens);
    parse_statement(&mut parser)
}
