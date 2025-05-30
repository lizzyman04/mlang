use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::{ASTNode, Expression};
use crate::core::parser::parse::parser::Parser;

pub fn parse_expression(parser: &mut Parser) -> Result<ASTNode, String> {
    parse_term(parser)
}

fn parse_term(parser: &mut Parser) -> Result<ASTNode, String> {
    let mut node = parse_factor(parser)?;

    while let Some(token) = parser.peek() {
        match &token.kind {
            TokenKind::SimpleSymbol(SimpleSymbolKind::Plus)
            | TokenKind::SimpleSymbol(SimpleSymbolKind::Minus)
            | TokenKind::MathSymbol(_) => {
                let operator = parser.advance().unwrap().clone();
                let right = parse_factor(parser)?;
                node = ASTNode::Expression(Expression::Binary {
                    left: Box::new(extract_expr(node)?),
                    operator,
                    right: Box::new(extract_expr(right)?),
                });
            }
            _ => break,
        }
    }

    Ok(node)
}

fn parse_factor(parser: &mut Parser) -> Result<ASTNode, String> {
    let mut node = parse_primary(parser)?;

    while let Some(token) = parser.peek() {
        match &token.kind {
            TokenKind::SimpleSymbol(SimpleSymbolKind::Star)
            | TokenKind::SimpleSymbol(SimpleSymbolKind::Slash)
            | TokenKind::MathSymbol(_) => {
                let operator = parser.advance().unwrap().clone();
                let right = parse_primary(parser)?;
                node = ASTNode::Expression(Expression::Binary {
                    left: Box::new(extract_expr(node)?),
                    operator,
                    right: Box::new(extract_expr(right)?),
                });
            }
            _ => break,
        }
    }

    Ok(node)
}

fn parse_primary(parser: &mut Parser) -> Result<ASTNode, String> {
    if let Some(token) = parser.advance() {
        match &token.kind {
            TokenKind::Int(i) => Ok(ASTNode::Expression(Expression::IntLiteral(*i))),
            TokenKind::Dec(f) => Ok(ASTNode::Expression(Expression::DecLiteral(*f))),
            TokenKind::Bool(b) => Ok(ASTNode::Expression(Expression::BoolLiteral(*b))),
            TokenKind::Txt(s) => Ok(ASTNode::Expression(Expression::TxtLiteral(s.clone()))),
            TokenKind::Identifier(id) => {
                Ok(ASTNode::Expression(Expression::Identifier(id.clone())))
            }
            TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen) => {
                let expr = parse_expression(parser)?;
                parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen))?;
                Ok(expr)
            }
            _ => Err(format!(
                "Unexpected token {:?} at line {}, column {}",
                token.kind, token.line, token.column
            )),
        }
    } else {
        Err("Unexpected end of input.".to_string())
    }
}

fn extract_expr(node: ASTNode) -> Result<Expression, String> {
    if let ASTNode::Expression(expr) = node {
        Ok(expr)
    } else {
        Err("Expected expression node".to_string())
    }
}
