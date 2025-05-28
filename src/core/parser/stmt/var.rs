use crate::core::lexer::token::TokenKind;
use crate::core::parser::parse::parser::Parser;
use crate::core::parser::ast::{ASTNode, Expression};

// pub fn parse_var_decl(parser: &mut Parser) -> Result<ASTNode, String> {
//     let var_type_token = parser.advance().ok_or("Expected type keyword")?;
//     let var_type = match &var_type_token.kind {
//         TokenKind::Keyword(t) => t.clone(),
//         _ => return Err("Expected type keyword".to_string()),
//     };

//     let name_token = parser.advance().ok_or("Expected variable name")?;
//     let name = match &name_token.kind {
//         TokenKind::Identifier(n) => n.clone(),
//         _ => return Err("Expected identifier for variable name".to_string()),
//     };

//     parser.consume(&TokenKind::Equal)?;
//     let expr = parse_expression(parser)?;
//     parser.consume(&TokenKind::Semicolon)?;

//     Ok(ASTNode::VarDecl {
//         var_type,
//         name,
//         value: Box::new(expr),
//     })
// }

pub fn parse_var_decl_with_name_and_type(
    parser: &mut Parser,
    var_type: &str,
    name: &str,
) -> Result<ASTNode, String> {
    parser.consume(&TokenKind::Equal)?;
    let expr = parse_expression(parser)?;
    parser.consume(&TokenKind::Semicolon)?;

    Ok(ASTNode::VarDecl {
        var_type: var_type.to_string(),
        name: name.to_string(),
        value: Box::new(expr),
    })
}

pub fn parse_expression(parser: &mut Parser) -> Result<ASTNode, String> {
    if let Some(token) = parser.advance() {
        match &token.kind {
            TokenKind::Int(i) => Ok(ASTNode::Expression(Expression::IntLiteral(*i))),
            TokenKind::Dec(f) => Ok(ASTNode::Expression(Expression::DecLiteral(*f))),
            TokenKind::Bool(b) => Ok(ASTNode::Expression(Expression::BoolLiteral(*b))),
            TokenKind::Txt(s) => Ok(ASTNode::Expression(Expression::TxtLiteral(s.clone()))),
            TokenKind::Identifier(id) => Ok(ASTNode::Expression(Expression::Identifier(id.clone()))),
            _ => Err(format!(
                "Expected literal but found {:?} at line {}, column {}",
                token.kind, token.line, token.column
            )),
        }
    } else {
        Err("Unexpected end of input in expression.".to_string())
    }
}
