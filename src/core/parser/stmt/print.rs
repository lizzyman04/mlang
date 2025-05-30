use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::expr::parse_expression;
use crate::core::parser::parse::parser::Parser;
use crate::core::lexer::token::TokenKind;

pub fn parse_print_stmt(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("print")?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen))?;
    let expr = parse_expression(parser)?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen))?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;

    Ok(ASTNode::PrintStmt {
        expression: Box::new(expr),
    })
}
