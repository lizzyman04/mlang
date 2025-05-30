use crate::core::parser::{ast::ASTNode, parse::{expr::parse_expression, parser::Parser}};

pub fn parse_return_stmt(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.advance();
    let expr = parse_expression(parser)?;
    Ok(ASTNode::ReturnStmt {
        value: Box::new(expr),
    })
}
