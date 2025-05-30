use super::env::Environment;
use super::eval::evaluate;
use crate::core::parser::ast::expr::ExecutionResult;
use crate::core::parser::ast::{ASTNode, Expression};

pub fn execute_stmt(stmt: ASTNode, env: &mut Environment) -> Result<ExecutionResult, String> {
    match stmt {
        ASTNode::PrintStmt { expression } => {
            if let ASTNode::Expression(expr) = *expression {
                let value = evaluate(expr, env)?;
                match value {
                    Expression::IntLiteral(i) => println!("{}", i),
                    Expression::DecLiteral(f) => println!("{}", f),
                    Expression::BoolLiteral(b) => println!("{}", b),
                    Expression::TxtLiteral(s) => println!("{}", s),
                    _ => return Err("Unsupported expression type in print.".to_string()),
                }
                Ok(ExecutionResult::Unit)
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
                let evaluated = match evaluate(expr.clone(), env) {
                    Ok(val) => val,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let matches_type = match (&var_type[..], &evaluated) {
                    ("int", Expression::IntLiteral(_)) => true,
                    ("dec", Expression::DecLiteral(_)) => true,
                    ("txt", Expression::TxtLiteral(_)) => true,
                    ("bool", Expression::BoolLiteral(_)) => true,
                    _ => false,
                };

                if !matches_type {
                    return Err(format!(
                        "Type mismatch: variable '{}' declared as '{}' but assigned incompatible value.",
                        name, var_type
                    ));
                }

                env.set(name, var_type, evaluated);
                Ok(ExecutionResult::Unit)
            } else {
                Err("Invalid expression in variable declaration.".to_string())
            }
        }

        ASTNode::ReturnStmt { value } => {
            if let ASTNode::Expression(expr) = *value {
                let result = evaluate(expr, env)?;
                Ok(ExecutionResult::Return(result))
            } else {
                Err("Invalid expression in return statement.".to_string())
            }
        }

        _ => Err("Unsupported statement.".to_string()),
    }
}
