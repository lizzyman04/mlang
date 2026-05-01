use super::parser::parse_statement;
use crate::core::lexer::symbol::comparison::ComparisonSymbolKind;
use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::{ASTNode, Type};
use crate::core::parser::parse::parser::Parser;

/// Parse a type annotation from the current parser position (advances past the type).
pub fn parse_type_annotation(parser: &mut Parser) -> Result<Type, String> {
    let kw = match parser.peek().map(|t| t.kind.clone()) {
        Some(TokenKind::Keyword(k)) => k,
        Some(other) => {
            return Err(format!(
                "Expected type keyword, found {}",
                other.display()
            ))
        }
        None => return Err("Expected type keyword, found end of input".to_string()),
    };
    parser.advance();
    parse_full_type(parser, &kw)
}

/// Given the already-consumed base keyword, parse the remainder of a type
/// (i.e. for `array` consume `<inner_type>`).
pub fn parse_full_type(parser: &mut Parser, base: &str) -> Result<Type, String> {
    match base {
        "int" => Ok(Type::Int),
        "dec" => Ok(Type::Dec),
        "txt" => Ok(Type::Txt),
        "bool" => Ok(Type::Bool),
        "void" => Ok(Type::Void),
        "array" => {
            parser.consume(&TokenKind::ComparisonSymbol(ComparisonSymbolKind::Less))?;
            let inner = parse_type_annotation(parser)?;
            parser.consume(&TokenKind::ComparisonSymbol(ComparisonSymbolKind::Greater))?;
            Ok(Type::Array(Box::new(inner)))
        }
        other => Err(format!("Unknown type '{}'", other)),
    }
}

pub fn parse_function_decl(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("main")?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen))?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen))?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftBrace))?;

    let mut body = Vec::new();
    while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace)) {
        body.push(parse_statement(parser)?);
    }
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace))?;

    Ok(ASTNode::FunctionDecl {
        name: "main".to_string(),
        return_type: Type::Void,
        params: vec![],
        body,
    })
}

pub fn parse_function_decl_with_signature(
    parser: &mut Parser,
    return_type: Type,
    name: &str,
) -> Result<ASTNode, String> {
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftParen))?;

    let mut params = Vec::new();
    while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen)) {
        let param_type = parse_type_annotation(parser)?;

        let param_name = {
            let tok = parser.advance().ok_or("Expected parameter name")?;
            match &tok.kind {
                TokenKind::Identifier(n) => n.clone(),
                _ => return Err("Expected identifier for parameter name".to_string()),
            }
        };

        params.push((param_type, param_name));

        if parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::Comma)) {
            parser.advance();
        } else {
            break;
        }
    }

    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightParen))?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftBrace))?;

    let mut body = Vec::new();
    while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace)) {
        body.push(parse_statement(parser)?);
    }
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace))?;

    Ok(ASTNode::FunctionDecl {
        name: name.to_string(),
        return_type,
        params,
        body,
    })
}
