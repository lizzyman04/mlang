use crate::core::parser::ast::ASTNode;
use crate::core::parser::parser::Parser;
use crate::core::lexer::token::TokenKind;
use super::parser::parse_statement;

pub fn parse_function_decl(parser: &mut Parser) -> Result<ASTNode, String> {
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
