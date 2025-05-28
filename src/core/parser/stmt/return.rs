use crate::core::parser::{ast::ASTNode, parse::parser::Parser};
use super::var::parse_expression;

pub fn parse_return_stmt(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.advance();
    let expr = parse_expression(parser)?;
    Ok(ASTNode::ReturnStmt {
        value: Box::new(expr),
    })
}
