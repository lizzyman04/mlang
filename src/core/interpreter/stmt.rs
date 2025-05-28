use crate::core::parser::ast::{ASTNode, Expression};
use super::env::Environment;
use super::eval::eval_expression;

pub fn execute_stmt(stmt: ASTNode, env: &mut Environment) -> Result<(), String> {
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

        ASTNode::VarDecl {
            var_type,
            name,
            value,
        } => {
            if let ASTNode::Expression(expr) = *value {
                let matches_type = match (&var_type[..], &expr) {
                    ("int", Expression::IntLiteral(_)) => true,
                    ("dec", Expression::DecLiteral(_)) => true,
                    ("txt", Expression::TxtLiteral(_)) => true,
                    _ => false,
                };

                if !matches_type {
                    return Err(format!(
                        "Type mismatch: variable '{}' declared as '{}' but assigned incompatible value.",
                        name, var_type
                    ));
                }

                env.set(name, var_type, expr);
                Ok(())
            } else {
                Err("Invalid expression in variable declaration.".to_string())
            }
        }

        _ => Err("Unsupported statement.".to_string()),
    }
}
