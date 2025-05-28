use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::parser::Parser;
use crate::core::parser::stmt::var::parse_expression;
use crate::core::lexer::token::TokenKind;

pub fn parse_print_stmt(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("print")?;
    parser.consume(&TokenKind::LeftParen)?;
    let expr = parse_expression(parser)?;
    parser.consume(&TokenKind::RightParen)?;
    parser.consume(&TokenKind::Semicolon)?;

    Ok(ASTNode::PrintStmt {
        expression: Box::new(expr),
    })
}
