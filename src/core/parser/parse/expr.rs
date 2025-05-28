use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::Expression;
use crate::core::parser::parse::parser::Parser;

pub fn parse_expression(parser: &mut Parser) -> Result<Expression, String> {
    parse_term(parser)
}

fn parse_term(parser: &mut Parser) -> Result<Expression, String> {
    let mut expr = parse_factor(parser)?;

    while let Some(token) = parser.peek() {
        match token.kind {
            TokenKind::Plus | TokenKind::Minus => {
                let op = parser.advance().unwrap().kind.clone();
                let right = parse_factor(parser)?;
                expr = Expression::Binary {
                    left: Box::new(expr),
                    operator: op,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_factor(parser: &mut Parser) -> Result<Expression, String> {
    let mut expr = parse_primary(parser)?;

    while let Some(token) = parser.peek() {
        match token.kind {
            TokenKind::Star | TokenKind::Slash => {
                let op = parser.advance().unwrap().kind.clone();
                let right = parse_primary(parser)?;
                expr = Expression::Binary {
                    left: Box::new(expr),
                    operator: op,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_primary(parser: &mut Parser) -> Result<Expression, String> {
    let token = parser.advance().ok_or("Unexpected end of input")?;
    match &token.kind {
        TokenKind::Int(i) => Ok(Expression::IntLiteral(*i)),
        TokenKind::Dec(d) => Ok(Expression::DecLiteral(*d)),
        TokenKind::Txt(s) => Ok(Expression::TxtLiteral(s.clone())),
        TokenKind::Bool(b) => Ok(Expression::BoolLiteral(*b)),
        TokenKind::Identifier(name) => Ok(Expression::Identifier(name.clone())),
        TokenKind::LeftParen => {
            let expr = parse_expression(parser)?;
            parser.consume(&TokenKind::RightParen)?;
            Ok(expr)
        }
        other => Err(format!(
            "Unexpected token {:?} at line {}, column {}",
            other, token.line, token.column
        )),
    }
}
