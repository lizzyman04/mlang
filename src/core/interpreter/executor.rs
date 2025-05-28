use crate::core::parser::ast::ASTNode;

use super::env::Environment;
use super::stmt::execute_stmt;

pub fn execute(ast: ASTNode) -> Result<(), String> {
    if let ASTNode::FunctionDecl { name, body } = ast {
        if name == "main" {
            let mut env = Environment::new();
            for stmt in body {
                execute_stmt(stmt, &mut env)?;
            }
            Ok(())
        } else {
            Err("No entry point found (expected 'main')".to_string())
        }
    } else {
        Err("Invalid program structure.".to_string())
    }
}
