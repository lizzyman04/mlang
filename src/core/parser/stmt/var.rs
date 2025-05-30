use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::parse::expr::parse_expression;
use crate::core::parser::parse::parser::Parser;
use crate::core::parser::ast::ASTNode;

pub fn parse_var_decl_with_name_and_type(
    parser: &mut Parser,
    var_type: &str,
    name: &str,
) -> Result<ASTNode, String> {
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Equal))?;
    let expr = parse_expression(parser)?;
    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::Semicolon))?;

    Ok(ASTNode::VarDecl {
        var_type: var_type.to_string(),
        name: name.to_string(),
        value: Box::new(expr),
    })
}