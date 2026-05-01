use super::env::Environment;
use super::eval::{coerce_value, evaluate};
use crate::core::lexer::rules::infer_type;
use crate::core::parser::ast::expr::ExecutionResult;
use crate::core::parser::ast::{ASTNode, Expression, Type};

pub fn execute_stmt(
    stmt: ASTNode,
    env: &mut Environment,
    output: Option<&mut &mut String>,
) -> Result<ExecutionResult, String> {
    match stmt {
        ASTNode::PrintStmt { expression } => {
            if let ASTNode::Expression(expr) = *expression {
                let value = evaluate(expr, env)?;
                let text = format_value(&value)?;
                if let Some(out) = output {
                    out.push_str(&format!("{}\n", text));
                } else {
                    println!("{}", text);
                }
                Ok(ExecutionResult::Unit)
            } else {
                Err("Expected expression in print.".to_string())
            }
        }

        ASTNode::VarDecl { var_type, name, value } => {
            if let ASTNode::Expression(expr) = *value {
                let evaluated = evaluate(expr, env)?;
                let coerced = coerce_to_type(&var_type, evaluated, &name)?;
                env.set(name, var_type, coerced);
                Ok(ExecutionResult::Unit)
            } else {
                Err("Invalid expression in variable declaration.".to_string())
            }
        }

        ASTNode::LetDecl { name, value } => {
            if let ASTNode::Expression(expr) = *value {
                let evaluated = evaluate(expr, env)?;
                let inferred = infer_type(&evaluated);
                env.set(name, inferred, evaluated);
                Ok(ExecutionResult::Unit)
            } else {
                Err("Invalid expression in let declaration.".to_string())
            }
        }

        ASTNode::IndexAssign { name, index, value } => {
            let idx = match *index {
                ASTNode::Expression(expr) => match evaluate(expr, env)? {
                    Expression::IntLiteral(i) if i >= 0 => i as usize,
                    Expression::IntLiteral(i) => {
                        return Err(format!("Array index cannot be negative: {}", i))
                    }
                    _ => return Err("Array index must be an integer".to_string()),
                },
                _ => return Err("Invalid index expression".to_string()),
            };

            let new_val = match *value {
                ASTNode::Expression(expr) => evaluate(expr, env)?,
                _ => return Err("Invalid value in index assignment".to_string()),
            };

            let (arr_type, arr_val) = {
                let r = env
                    .get(&name)
                    .ok_or_else(|| format!("Undefined variable '{}'", name))?;
                (r.0.clone(), r.1.clone())
            };

            let inner_type = match &arr_type {
                Type::Array(inner) => *inner.clone(),
                _ => return Err(format!("'{}' is not an array", name)),
            };

            match arr_val {
                Expression::ArrayLiteral(mut elems) => {
                    if idx >= elems.len() {
                        return Err(format!(
                            "Index {} out of bounds for array of length {}",
                            idx,
                            elems.len()
                        ));
                    }
                    elems[idx] = coerce_value(&inner_type, new_val)?;
                    env.set(name, arr_type, Expression::ArrayLiteral(elems));
                    Ok(ExecutionResult::Unit)
                }
                _ => Err(format!("'{}' is not an array", name)),
            }
        }

        ASTNode::ExprStmt(expr_node) => {
            if let ASTNode::Expression(expr) = *expr_node {
                evaluate(expr, env)?;
            }
            Ok(ExecutionResult::Unit)
        }

        ASTNode::ReturnStmt { value } => {
            if let ASTNode::Expression(expr) = *value {
                Ok(ExecutionResult::Return(evaluate(expr, env)?))
            } else {
                Err("Invalid expression in return statement.".to_string())
            }
        }

        _ => Err("Unsupported statement.".to_string()),
    }
}

fn format_value(value: &Expression) -> Result<String, String> {
    match value {
        Expression::IntLiteral(i) => Ok(i.to_string()),
        Expression::DecLiteral(f) => Ok(f.to_string()),
        Expression::BoolLiteral(b) => Ok(b.to_string()),
        Expression::TxtLiteral(s) => Ok(s.clone()),
        Expression::ArrayLiteral(elems) => {
            let parts: Vec<String> = elems
                .iter()
                .map(format_value)
                .collect::<Result<_, _>>()?;
            Ok(format!("[{}]", parts.join(", ")))
        }
        _ => Err("Unsupported value type in print.".to_string()),
    }
}

fn coerce_to_type(expected: &Type, value: Expression, var_name: &str) -> Result<Expression, String> {
    match expected {
        Type::Int => match value {
            Expression::IntLiteral(_) => Ok(value),
            _ => mismatch(expected, &value, var_name),
        },
        Type::Dec => match value {
            Expression::DecLiteral(_) => Ok(value),
            Expression::IntLiteral(i) => Ok(Expression::DecLiteral(i as f64)),
            _ => mismatch(expected, &value, var_name),
        },
        Type::Txt => match value {
            Expression::TxtLiteral(_) => Ok(value),
            _ => mismatch(expected, &value, var_name),
        },
        Type::Bool => match value {
            Expression::BoolLiteral(_) => Ok(value),
            _ => mismatch(expected, &value, var_name),
        },
        Type::Array(inner) => match value {
            Expression::ArrayLiteral(elems) => {
                let coerced = elems
                    .into_iter()
                    .map(|e| coerce_to_type(inner, e, var_name))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Expression::ArrayLiteral(coerced))
            }
            _ => mismatch(expected, &value, var_name),
        },
        Type::Void => Ok(value),
    }
}

fn mismatch(expected: &Type, got: &Expression, var_name: &str) -> Result<Expression, String> {
    Err(format!(
        "Type mismatch: variable '{}' declared as '{}' but got '{}'.",
        var_name,
        expected,
        infer_type(got)
    ))
}
