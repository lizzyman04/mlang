use crate::core::lexer::token::Token;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::stmt::parse_statement;
use crate::core::parser::parse::parser::Parser;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<ASTNode>, String> {
    let mut parser = Parser::new(tokens);
    let mut nodes = Vec::new();

    while !parser.is_at_end() {
        nodes.push(parse_statement(&mut parser)?);
    }

    Ok(nodes)
}