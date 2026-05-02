use crate::core::lexer::symbol::simple::SimpleSymbolKind;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::parser::Parser;
use super::func::parse_type_annotation;

pub fn parse_struct_decl(parser: &mut Parser) -> Result<ASTNode, String> {
    parser.consume_keyword("struct")?;

    let name = {
        let tok = parser.consume_identifier()?;
        match &tok.kind {
            TokenKind::Identifier(n) => n.clone(),
            _ => unreachable!(),
        }
    };

    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::LeftBrace))?;

    let mut fields = Vec::new();
    while !parser.check(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace)) {
        let field_type = parse_type_annotation(parser)?;
        let field_name = {
            let tok = parser.advance().ok_or("Expected field name in struct")?;
            match &tok.kind {
                TokenKind::Identifier(n) => n.clone(),
                _ => {
                    return Err(format!(
                        "Expected field name, found {}",
                        tok.kind.display()
                    ))
                }
            }
        };
        fields.push((field_type, field_name));
    }

    parser.consume(&TokenKind::SimpleSymbol(SimpleSymbolKind::RightBrace))?;

    Ok(ASTNode::StructDecl { name, fields })
}
