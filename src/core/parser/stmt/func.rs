use super::parser::parse_statement;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::parser::Parser;

pub fn parse_function_decl(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("main")?;
    parser.consume(&TokenKind::LeftParen)?;
    parser.consume(&TokenKind::RightParen)?;
    parser.consume(&TokenKind::LeftBrace)?;

    let mut body = Vec::new();
    while !parser.check(&TokenKind::RightBrace) {
        body.push(parse_statement(parser)?);
    }

    parser.consume(&TokenKind::RightBrace)?;

    Ok(ASTNode::FunctionDecl {
        name: "main".to_string(),
        return_type: "void".to_string(),
        params: vec![],
        body,
    })
}

pub fn parse_function_decl_with_signature(
    parser: &mut Parser,
    return_type: &str,
    name: &str,
) -> Result<ASTNode, String> {
    parser.consume(&TokenKind::LeftParen)?;

    let mut params = Vec::new();

    while !parser.check(&TokenKind::RightParen) {
        let type_token = parser.advance().ok_or("Expected parameter type")?;
        let param_type = match &type_token.kind {
            TokenKind::Keyword(t) => t.clone(),
            _ => return Err("Expected type for parameter".to_string()),
        };

        let name_token = parser.advance().ok_or("Expected parameter name")?;
        let param_name = match &name_token.kind {
            TokenKind::Identifier(n) => n.clone(),
            _ => return Err("Expected name for parameter".to_string()),
        };

        params.push((param_type, param_name));

        if parser.check(&TokenKind::Comma) {
            parser.advance();
        } else {
            break;
        }
    }

    parser.consume(&TokenKind::RightParen)?;
    parser.consume(&TokenKind::LeftBrace)?;

    let mut body = Vec::new();
    while !parser.check(&TokenKind::RightBrace) {
        body.push(parse_statement(parser)?);
    }

    parser.consume(&TokenKind::RightBrace)?;

    Ok(ASTNode::FunctionDecl {
        name: name.to_string(),
        return_type: return_type.to_string(),
        params,
        body,
    })
}
