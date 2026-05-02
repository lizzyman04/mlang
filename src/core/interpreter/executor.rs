use super::env::{Environment, FunctionDef};
use super::stmt::execute_stmt;
use crate::core::lexer::rules::infer_type;
use crate::core::parser::ast::expr::ExecutionResult;
use crate::core::parser::ast::{ASTNode, Expression, Type};

pub fn execute_repl_stmts(stmts: Vec<ASTNode>, env: &mut Environment) -> Result<(), String> {
    for stmt in stmts {
        match execute_stmt(stmt, env, None)? {
            ExecutionResult::Return(value) => {
                println!("{}", repl_display(&value));
                break;
            }
            ExecutionResult::Break => {
                return Err("'break' used outside of a loop".to_string())
            }
            ExecutionResult::Continue => {
                return Err("'continue' used outside of a loop".to_string())
            }
            ExecutionResult::Unit => continue,
        }
    }
    Ok(())
}

pub fn execute(program: Vec<ASTNode>, mut output: Option<&mut String>) -> Result<(), String> {
    let mut env = Environment::new();

    // First pass: register all non-main functions
    for node in &program {
        if let ASTNode::FunctionDecl { name, body, return_type, params } = node {
            if name != "main" {
                env.register_function(name.clone(), FunctionDef {
                    params: params.clone(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                });
            }
        }
    }

    for node in program {
        if let ASTNode::FunctionDecl { name, body, return_type, params } = node {
            if name == "main" {
                if !params.is_empty() {
                    return Err("Function 'main' should not have parameters".to_string());
                }
                for stmt in body {
                    match execute_stmt(stmt, &mut env, output.as_mut().map(|o| o))? {
                        ExecutionResult::Return(value) => {
                            if return_type != Type::Void {
                                let got = infer_type(&value);
                                if return_type != got {
                                    return Err(format!(
                                        "Return type mismatch: expected '{}', got '{}'",
                                        return_type, got
                                    ));
                                }
                            }
                            return Ok(());
                        }
                        ExecutionResult::Break => {
                            return Err("'break' used outside of a loop".to_string())
                        }
                        ExecutionResult::Continue => {
                            return Err("'continue' used outside of a loop".to_string())
                        }
                        ExecutionResult::Unit => continue,
                    }
                }
                return Ok(());
            }
        }
    }

    Err("No entry point found (expected 'main')".to_string())
}

fn repl_display(value: &Expression) -> String {
    match value {
        Expression::IntLiteral(i) => format!("= {}", i),
        Expression::DecLiteral(f) => format!("= {}", f),
        Expression::TxtLiteral(s) => format!("= \"{}\"", s),
        Expression::BoolLiteral(b) => format!("= {}", b),
        Expression::ArrayLiteral(elems) => {
            let parts: Vec<String> = elems.iter().map(repl_display).collect();
            format!("= [{}]", parts.join(", "))
        }
        _ => "= <value>".to_string(),
    }
}
