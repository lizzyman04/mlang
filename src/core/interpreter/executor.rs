use crate::core::parser::ast::{ASTNode, Expression};
use std::collections::HashMap;

pub fn execute(ast: ASTNode) -> Result<(), String> {
    if let ASTNode::FunctionDecl { name, body } = ast {
        if name == "main" {
            let mut env = HashMap::new();
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

fn execute_stmt(stmt: ASTNode, env: &mut HashMap<String, Expression>) -> Result<(), String> {
    match stmt {
        ASTNode::PrintStmt { expression } => {
            if let ASTNode::Expression(expr) = *expression {
                let value = eval_expression(expr, env)?;
                println!("{}", value);
                Ok(())
            } else {
                Err("Expected expression in print.".to_string())
            }
        }
        ASTNode::VarDecl { name, value, .. } => {
            if let ASTNode::Expression(expr) = *value {
                env.insert(name, expr);
                Ok(())
            } else {
                Err("Invalid expression in variable declaration.".to_string())
            }
        }
        _ => Err("Unsupported statement.".to_string()),
    }
}

fn eval_expression(expr: Expression, env: &HashMap<String, Expression>) -> Result<String, String> {
    match expr {
        Expression::IntLiteral(i) => Ok(i.to_string()),
        Expression::DecLiteral(f) => Ok(f.to_string()),
        Expression::TxtLiteral(s) => Ok(s),
        Expression::Identifier(name) => {
            if let Some(value) = env.get(&name) {
                eval_expression(value.clone(), env)
            } else {
                Err(format!("Undefined variable '{}'", name))
            }
        }
    }
}
