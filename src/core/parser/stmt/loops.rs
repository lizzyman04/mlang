use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::expr::parse_expression;
use crate::core::parser::parse::parser::Parser;
use super::parser::parse_statement;

pub fn parse_while_loop(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("while")?;
    let condition = parse_expression(parser)?;
    let body = parse_block(parser)?;
    Ok(ASTNode::WhileLoop { condition: Box::new(condition), body })
}

pub fn parse_for_loop(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("for")?;
    let var = {
        let tok = parser.consume_identifier()?;
        match &tok.kind {
            TokenKind::Identifier(n) => n.clone(),
            _ => unreachable!(),
        }
    };
    parser.consume_keyword("in")?;
    let first_expr = parse_expression(parser)?;

    if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::DotDot)) {
        parser.advance();
        let end_expr = parse_expression(parser)?;
        let body = parse_block(parser)?;
        Ok(ASTNode::ForRangeLoop {
            var,
            start: Box::new(first_expr),
            end: Box::new(end_expr),
            body,
        })
    } else {
        let body = parse_block(parser)?;
        Ok(ASTNode::ForArrayLoop {
            var,
            array: Box::new(first_expr),
            body,
        })
    }
}

pub fn parse_if_stmt(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("if")?;
    let condition = parse_expression(parser)?;
    let then_body = parse_block(parser)?;
    let else_body = if parser.check(&TokenKind::Keyword("else".to_string())) {
        parser.advance();
        Some(parse_block(parser)?)
    } else {
        None
    };
    Ok(ASTNode::IfStmt { condition: Box::new(condition), then_body, else_body })
}

fn parse_block(parser: &mut Parser) -> Result<Vec<ASTNode>, String> {
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftBrace))?;
    let mut body = Vec::new();
    while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace)) {
        body.push(parse_statement(parser)?);
    }
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace))?;
    Ok(body)
}
