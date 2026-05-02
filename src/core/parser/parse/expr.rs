use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::{ASTNode, Expression};
use crate::core::parser::parse::parser::Parser;

pub fn parse_expression(parser: &mut Parser) -> Result<ASTNode, String> {
    parse_or(parser)
}

// || (lowest precedence)
fn parse_or(parser: &mut Parser) -> Result<ASTNode, String> {
    let mut node = parse_and(parser)?;

    while let Some(token) = parser.peek() {
        if token.kind != TokenKind::SimpleSymbol(SimpleSymbolKind::Or) {
            break;
        }
        let operator = parser.advance().unwrap().clone();
        let right = parse_and(parser)?;
        node = ASTNode::Expression(Expression::Logical {
            left: Box::new(extract_expr(node)?),
            operator,
            right: Box::new(extract_expr(right)?),
        });
    }

    Ok(node)
}

// &&
fn parse_and(parser: &mut Parser) -> Result<ASTNode, String> {
    let mut node = parse_comparison(parser)?;

    while let Some(token) = parser.peek() {
        if token.kind != TokenKind::SimpleSymbol(SimpleSymbolKind::And) {
            break;
        }
        let operator = parser.advance().unwrap().clone();
        let right = parse_comparison(parser)?;
        node = ASTNode::Expression(Expression::Logical {
            left: Box::new(extract_expr(node)?),
            operator,
            right: Box::new(extract_expr(right)?),
        });
    }

    Ok(node)
}

// ==, !=, <, >, <=, >=
fn parse_comparison(parser: &mut Parser) -> Result<ASTNode, String> {
    let mut node = parse_term(parser)?;

    while let Some(token) = parser.peek() {
        if !matches!(token.kind, TokenKind::ComparisonSymbol(_)) {
            break;
        }
        let operator = parser.advance().unwrap().clone();
        let right = parse_term(parser)?;
        node = ASTNode::Expression(Expression::Binary {
            left: Box::new(extract_expr(node)?),
            operator,
            right: Box::new(extract_expr(right)?),
        });
    }

    Ok(node)
}

// +, -
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

// *, /
fn parse_factor(parser: &mut Parser) -> Result<ASTNode, String> {
    let mut node = parse_unary(parser)?;

    while let Some(token) = parser.peek() {
        match &token.kind {
            TokenKind::SimpleSymbol(SimpleSymbolKind::Star)
            | TokenKind::SimpleSymbol(SimpleSymbolKind::Slash)
            | TokenKind::MathSymbol(_) => {
                let operator = parser.advance().unwrap().clone();
                let right = parse_unary(parser)?;
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

// !, unary -
fn parse_unary(parser: &mut Parser) -> Result<ASTNode, String> {
    if let Some(token) = parser.peek() {
        match &token.kind {
            TokenKind::SimpleSymbol(SimpleSymbolKind::Not)
            | TokenKind::SimpleSymbol(SimpleSymbolKind::Minus) => {
                let operator = parser.advance().unwrap().clone();
                let operand = parse_unary(parser)?;
                return Ok(ASTNode::Expression(Expression::Unary {
                    operator,
                    operand: Box::new(extract_expr(operand)?),
                }));
            }
            _ => {}
        }
    }
    parse_postfix(parser)
}

// expr[index], expr.method(args)
fn parse_postfix(parser: &mut Parser) -> Result<ASTNode, String> {
    let mut node = parse_primary(parser)?;

    loop {
        match parser.peek().map(|t| &t.kind) {
            Some(TokenKind::SimpleSymbol(SimpleSymbolKind::LeftBracket)) => {
                parser.advance();
                let index_node = parse_expression(parser)?;
                let index = extract_expr(index_node)?;
                parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBracket))?;
                let array = extract_expr(node)?;
                node = ASTNode::Expression(Expression::ArrayAccess {
                    array: Box::new(array),
                    index: Box::new(index),
                });
            }
            Some(TokenKind::SimpleSymbol(SimpleSymbolKind::Dot)) => {
                parser.advance();
                let member = {
                    let tok = parser.advance().ok_or("Expected field or method name after '.'")?;
                    match &tok.kind {
                        TokenKind::Identifier(m) => m.clone(),
                        _ => {
                            return Err(format!(
                                "Expected field or method name after '.', found {}",
                                tok.kind.display()
                            ))
                        }
                    }
                };
                let object = extract_expr(node)?;
                if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen)) {
                    parser.advance();
                    let mut args = Vec::new();
                    while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen)) {
                        let arg = extract_expr(parse_expression(parser)?)?;
                        args.push(arg);
                        if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::Comma)) {
                            parser.advance();
                        } else {
                            break;
                        }
                    }
                    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen))?;
                    node = ASTNode::Expression(Expression::MethodCall {
                        object: Box::new(object),
                        method: member,
                        args,
                    });
                } else {
                    node = ASTNode::Expression(Expression::FieldAccess {
                        object: Box::new(object),
                        field: member,
                    });
                }
            }
            _ => break,
        }
    }

    Ok(node)
}

fn parse_primary(parser: &mut Parser) -> Result<ASTNode, String> {
    // Array literal
    if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftBracket)) {
        parser.advance();
        let mut elements = Vec::new();
        while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBracket)) {
            let elem = extract_expr(parse_expression(parser)?)?;
            elements.push(elem);
            if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::Comma)) {
                parser.advance();
            } else {
                break;
            }
        }
        parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBracket))?;
        return Ok(ASTNode::Expression(Expression::ArrayLiteral(elements)));
    }

    if let Some(token) = parser.advance() {
        match &token.kind {
            TokenKind::Int(i) => Ok(ASTNode::Expression(Expression::IntLiteral(*i))),
            TokenKind::Dec(f) => Ok(ASTNode::Expression(Expression::DecLiteral(*f))),
            TokenKind::Bool(b) => Ok(ASTNode::Expression(Expression::BoolLiteral(*b))),
            TokenKind::Txt(s) => Ok(ASTNode::Expression(Expression::TxtLiteral(s.clone()))),
            TokenKind::Identifier(id) => {
                let name = id.clone();
                if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen)) {
                    parser.advance(); // consume `(`
                    let mut args = Vec::new();
                    while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen)) {
                        args.push(extract_expr(parse_expression(parser)?)?);
                        if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::Comma)) {
                            parser.advance();
                        } else {
                            break;
                        }
                    }
                    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen))?;
                    Ok(ASTNode::Expression(Expression::FnCall { name, args }))
                } else if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftBrace))
                    && matches!(
                        parser.peek_ahead(1).map(|t| &t.kind),
                        Some(TokenKind::Identifier(_))
                    )
                    && matches!(
                        parser.peek_ahead(2).map(|t| &t.kind),
                        Some(TokenKind::SimpleSymbol(SimpleSymbolKind::Equal))
                    )
                {
                    parser.advance(); // consume `{`
                    let mut fields = Vec::new();
                    while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace)) {
                        let field_name = {
                            let tok = parser.consume_identifier()?;
                            match &tok.kind {
                                TokenKind::Identifier(n) => n.clone(),
                                _ => unreachable!(),
                            }
                        };
                        parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Equal))?;
                        let field_val = extract_expr(parse_expression(parser)?)?;
                        fields.push((field_name, field_val));
                        if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::Comma)) {
                            parser.advance();
                        } else {
                            break;
                        }
                    }
                    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace))?;
                    Ok(ASTNode::Expression(Expression::StructLiteral { name, fields }))
                } else {
                    Ok(ASTNode::Expression(Expression::Identifier(name)))
                }
            }
            TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen) => {
                let expr = parse_expression(parser)?;
                parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen))?;
                Ok(expr)
            }
            TokenKind::Keyword(kw) if matches!(kw.as_str(), "int" | "dec" | "txt") => {
                let name = kw.clone();
                if !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen)) {
                    return Err(format!(
                        "'{}' is a type keyword; use '{}(value)' to convert",
                        name, name
                    ));
                }
                parser.advance(); // consume `(`
                let mut args = Vec::new();
                while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen)) {
                    args.push(extract_expr(parse_expression(parser)?)?);
                    if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::Comma)) {
                        parser.advance();
                    } else {
                        break;
                    }
                }
                parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen))?;
                Ok(ASTNode::Expression(Expression::FnCall { name, args }))
            }
            other => Err(format!(
                "Unexpected token {} in expression",
                other.display()
            )),
        }
    } else {
        Err("Unexpected end of input in expression".to_string())
    }
}

pub fn extract_expr(node: ASTNode) -> Result<Expression, String> {
    if let ASTNode::Expression(expr) = node {
        Ok(expr)
    } else {
        Err("Expected expression node".to_string())
    }
}
