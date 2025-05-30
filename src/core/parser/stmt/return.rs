use crate::core::{
    lexer::{symbol::simple::SimpleSymbolKind, token::TokenKind},
    parser::{
        ast::ASTNode,
        parse::{expr::parse_expression, parser::Parser},
    },
};

pub fn parse_return_stmt(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.advance();
    let expr = parse_expression(parser)?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;
    Ok(ASTNode::ReturnStmt {
        value: Box::new(expr),
    })
}
