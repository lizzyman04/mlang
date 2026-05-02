use crate::core::lexer::token::Token;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::parser::Parser;
use crate::core::parser::stmt::parse_statement;

pub fn parse_stmts(tokens: Vec<Token>) -> Result<Vec<ASTNode>, String> {
    let mut parser = Parser::new(tokens);
    let mut stmts = Vec::new();

    while !parser.is_at_end() {
        stmts.push(parse_statement(&mut parser)?);
    }

    Ok(stmts)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<ASTNode>, String> {
    let mut parser = Parser::new(tokens);
    let mut nodes = Vec::new();
    let mut main_count = 0;

    while !parser.is_at_end() {
        let stmt = parse_statement(&mut parser)?;

        match &stmt {
            ASTNode::FunctionDecl { name, .. } => {
                if name == "main" {
                    main_count += 1;
                }
                nodes.push(stmt);
            }
            ASTNode::StructDecl { .. } => {
                nodes.push(stmt);
            }
            _ => {
                return Err(
                    "Only function and struct declarations are allowed at the top level."
                        .to_string(),
                )
            }
        }
    }

    match main_count {
        0 => Err("No entry point found (expected 'main')".to_string()),
        1 => Ok(nodes),
        _ => Err("Multiple 'main' functions found.".to_string()),
    }
}
