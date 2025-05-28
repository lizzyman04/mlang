use crate::core::lexer::token::Token;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::parser::Parser;
use crate::core::parser::stmt::parse_statement;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<ASTNode>, String> {
    let mut parser = Parser::new(tokens);
    let mut functions = Vec::new();
    let mut main_count = 0;

    while !parser.is_at_end() {
        let stmt = parse_statement(&mut parser)?;

        if let ASTNode::FunctionDecl { name, .. } = &stmt {
            if name == "main" {
                main_count += 1;
            }
            functions.push(stmt);
        } else {
            return Err("Only function declarations are allowed at the top level.".to_string());
        }
    }

    match main_count {
        0 => Err("No entry point found (expected 'main')".to_string()),
        1 => Ok(functions),
        _ => Err("Multiple 'main' functions found.".to_string()),
    }
}
