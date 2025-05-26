pub mod ast;
pub mod error;
pub mod helpers;
pub mod parser;
pub mod stmt;

pub use ast::ASTNode;
pub use parser::Parser;
pub use stmt::parse_statement;

pub fn parse(tokens: Vec<crate::core::lexer::token::Token>) -> Result<ASTNode, String> {
    let mut parser = Parser::new(tokens);
    parse_statement(&mut parser)
}
