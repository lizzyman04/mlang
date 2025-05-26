use crate::core::parser::ast::ASTNode;
use crate::core::parser::parser::Parser;
use crate::core::lexer::token::TokenKind;

pub fn parse_statement(parser: &mut Parser) -> Result<ASTNode, String> {
    if let Some(token) = parser.peek() {
        match &token.kind {
            TokenKind::Keyword(kw) if kw == "main" => parse_function_decl(parser),
            TokenKind::Keyword(kw) if kw == "print" => parse_print_stmt(parser),
            _ => Err(format!(
                "Unexpected token {:?} at line {}, column {}",
                token.kind, token.line, token.column
            )),
        }
    } else {
        Err("Unexpected end of input".to_string())
    }
}

fn parse_function_decl(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("main")?;
    parser.consume(&TokenKind::LeftParen)?;
    parser.consume(&TokenKind::RightParen)?;
    parser.consume(&TokenKind::LeftBrace)?;

    let mut body = Vec::new();
    while let Some(token) = parser.peek() {
        if let TokenKind::RightBrace = token.kind {
            break;
        }
        let stmt = parse_statement(parser)?;
        body.push(stmt);
    }

    parser.consume(&TokenKind::RightBrace)?;

    Ok(ASTNode::FunctionDecl {
        name: "main".to_string(),
        body,
    })
}

fn parse_print_stmt(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("print")?;
    parser.consume(&TokenKind::LeftParen)?;
    let expr = parse_expression(parser)?;
    parser.consume(&TokenKind::RightParen)?;
    parser.consume(&TokenKind::Semicolon)?;

    Ok(ASTNode::PrintStmt {
        expression: Box::new(expr),
    })
}

fn parse_expression(parser: &mut Parser) -> Result<ASTNode, String> {
    if let Some(token) = parser.advance() {
        match &token.kind {
            TokenKind::Int(n) => Ok(ASTNode::IntLiteral(*n)),
            TokenKind::Txt(s) => Ok(ASTNode::TxtLiteral(s.clone())),
            _ => Err(format!(
                "Expected literal but found {:?} at line {}, column {}",
                token.kind, token.line, token.column
            )),
        }
    } else {
        Err("Unexpected end of input while parsing expression".to_string())
    }
}
