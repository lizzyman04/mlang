use crate::core::lexer::rules::is_variable_type;
use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::{ASTNode, Expression};
use crate::core::parser::parse::expr::{extract_expr, parse_expression};
use crate::core::parser::parse::parser::Parser;

use super::decider::parse_var_or_function_decl;
use super::func::parse_function_decl;
use super::loops::{parse_for_loop, parse_if_stmt, parse_while_loop};
use super::print::parse_print_stmt;
use super::r#return::parse_return_stmt;

pub fn parse_statement(parser: &mut Parser) -> Result<ASTNode, String> {
    if let Some(token) = parser.peek() {
        match token.kind.clone() {
            TokenKind::Keyword(ref kw) if kw == "main" => parse_function_decl(parser),
            TokenKind::Keyword(ref kw) if kw == "print" => parse_print_stmt(parser),
            TokenKind::Keyword(ref kw) if kw == "return" => parse_return_stmt(parser),
            TokenKind::Keyword(ref kw) if kw == "if" => parse_if_stmt(parser),
            TokenKind::Keyword(ref kw) if kw == "while" => parse_while_loop(parser),
            TokenKind::Keyword(ref kw) if kw == "for" => parse_for_loop(parser),
            TokenKind::Keyword(ref kw) if kw == "break" => {
                parser.advance();
                parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;
                Ok(ASTNode::Break)
            }
            TokenKind::Keyword(ref kw) if kw == "continue" => {
                parser.advance();
                parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;
                Ok(ASTNode::Continue)
            }
            TokenKind::Keyword(ref kw) if kw == "let" => parse_let_decl(parser),
            TokenKind::Keyword(ref kw) if is_variable_type(kw) => {
                let base = kw.clone();
                parser.advance();
                parse_var_or_function_decl(parser, &base)
            }
            TokenKind::Identifier(_) => parse_ident_stmt(parser),
            _ => Err(format!(
                "Unexpected token {} at line {}, column {}",
                token.kind.display(),
                token.line,
                token.column
            )),
        }
    } else {
        Err("Unexpected end of input".to_string())
    }
}

fn parse_let_decl(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.advance(); // consume `let`
    let name = {
        let tok = parser.consume_identifier()?;
        match &tok.kind {
            TokenKind::Identifier(n) => n.clone(),
            _ => unreachable!(),
        }
    };
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Equal))?;
    let expr = parse_expression(parser)?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;
    Ok(ASTNode::LetDecl {
        name,
        value: Box::new(expr),
    })
}

/// Handles statements that start with an identifier:
/// - `name[index] = value;`  → IndexAssign
/// - `name.method(args);`    → ExprStmt (method call)
fn parse_ident_stmt(parser: &mut Parser) -> Result<ASTNode, String> {
    let name = {
        let tok = parser.advance().unwrap();
        match &tok.kind {
            TokenKind::Identifier(n) => n.clone(),
            _ => unreachable!(),
        }
    };

    match parser.peek().map(|t| t.kind.clone()) {
        Some(TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen)) => {
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
            parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;
            Ok(ASTNode::ExprStmt(Box::new(ASTNode::Expression(
                Expression::FnCall { name, args },
            ))))
        }
        Some(TokenKind::SimpleSymbol(SimpleSymbolKind::LeftBracket)) => {
            parser.advance(); // consume `[`
            let index = extract_expr(parse_expression(parser)?)?;
            parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBracket))?;

            if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::Equal)) {
                parser.advance(); // consume `=`
                let value = extract_expr(parse_expression(parser)?)?;
                parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;
                Ok(ASTNode::IndexAssign {
                    name,
                    index: Box::new(ASTNode::Expression(index)),
                    value: Box::new(ASTNode::Expression(value)),
                })
            } else {
                // Read-only index access as statement (no-op, but valid syntax)
                parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;
                Ok(ASTNode::ExprStmt(Box::new(ASTNode::Expression(
                    Expression::ArrayAccess {
                        array: Box::new(Expression::Identifier(name)),
                        index: Box::new(index),
                    },
                ))))
            }
        }
        Some(TokenKind::SimpleSymbol(SimpleSymbolKind::Dot)) => {
            parser.advance(); // consume `.`
            let method = {
                let tok = parser.advance().ok_or("Expected method name after '.'")?;
                match &tok.kind {
                    TokenKind::Identifier(m) => m.clone(),
                    _ => {
                        return Err(format!(
                            "Expected method name after '.', found {}",
                            tok.kind.display()
                        ))
                    }
                }
            };
            parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen))?;
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
            parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;
            Ok(ASTNode::ExprStmt(Box::new(ASTNode::Expression(
                Expression::MethodCall {
                    object: Box::new(Expression::Identifier(name)),
                    method,
                    args,
                },
            ))))
        }
        Some(TokenKind::SimpleSymbol(SimpleSymbolKind::Equal)) => {
            parser.advance();
            let value = extract_expr(parse_expression(parser)?)?;
            parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;
            Ok(ASTNode::VarAssign {
                name,
                value: Box::new(ASTNode::Expression(value)),
            })
        }
        Some(other) => Err(format!(
            "Expected '=', '[', or '.' after identifier '{}', found {}",
            name,
            other.display()
        )),
        None => Err(format!(
            "Unexpected end of input after identifier '{}'",
            name
        )),
    }
}
