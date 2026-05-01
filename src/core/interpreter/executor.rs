use super::env::Environment;
use super::stmt::execute_stmt;
use crate::core::lexer::rules::infer_type_expr;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::ast::expr::ExecutionResult;

pub fn execute_repl_stmts(stmts: Vec<ASTNode>, env: &mut Environment) -> Result<(), String> {
    use crate::core::parser::ast::Expression;

    for stmt in stmts {
        match execute_stmt(stmt, env, None)? {
            ExecutionResult::Return(value) => {
                let display = match &value {
                    Expression::IntLiteral(i) => format!("= {}", i),
                    Expression::DecLiteral(f) => format!("= {}", f),
                    Expression::TxtLiteral(s) => format!("= \"{}\"", s),
                    Expression::BoolLiteral(b) => format!("= {}", b),
                    _ => "= <value>".to_string(),
                };
                println!("{}", display);
                break;
            }
            _ => continue,
        }
    }
    Ok(())
}

pub fn execute(program: Vec<ASTNode>, mut output: Option<&mut String>) -> Result<(), String> {
    let mut env = Environment::new();

    for node in program {
        if let ASTNode::FunctionDecl {
            name,
            body,
            return_type,
            params,
        } = node
        {
            if name == "main" {
                if !params.is_empty() {
                    return Err("Function 'main' should not have parameters".to_string());
                }

                for stmt in body {
                    match execute_stmt(stmt, &mut env, output.as_mut().map(|o| o))? {
                        ExecutionResult::Return(value) => {
                            if return_type != "void" && return_type != infer_type_expr(&value) {
                                return Err(format!(
                                    "Return type mismatch: expected '{}', got '{}'",
                                    return_type,
                                    infer_type_expr(&value)
                                ));
                            }
                            return Ok(());
                        }
                        _ => continue,
                    }
                }
                return Ok(());
            }
        }
    }

    Err("No entry point found (expected 'main')".to_string())
}
