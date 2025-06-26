use super::env::Environment;
use super::stmt::execute_stmt;
use crate::core::lexer::rules::infer_type_expr;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::ast::expr::ExecutionResult;

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
