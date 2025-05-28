use crate::core::parser::ast::ASTNode;

use super::env::Environment;
use super::stmt::execute_stmt;

pub fn execute(program: Vec<ASTNode>) -> Result<(), String> {
    let mut env = Environment::new();

    for node in program {
        if let ASTNode::FunctionDecl { name, body } = node {
            if name == "main" {
                for stmt in body {
                    execute_stmt(stmt, &mut env)?;
                }
                return Ok(());
            }
        }
    }

    Err("No entry point found (expected 'main')".to_string())
}