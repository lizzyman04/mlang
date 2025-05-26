use crate::core::parser::ast::ASTNode;

pub fn execute(ast: ASTNode) -> Result<(), String> {
    match ast {
        ASTNode::FunctionDecl { name, body } if name == "main" => {
            for stmt in body {
                execute_stmt(stmt)?;
            }
            Ok(())
        }
        _ => Err("No entry point found (expected 'main' function).".to_string()),
    }
}

fn execute_stmt(stmt: ASTNode) -> Result<(), String> {
    match stmt {
        ASTNode::PrintStmt { expression } => {
            let value = eval_expression(*expression)?;
            println!("{}", value);
            Ok(())
        }
        _ => Err("Unsupported statement.".to_string()),
    }
}

fn eval_expression(expr: ASTNode) -> Result<String, String> {
    match expr {
        ASTNode::IntLiteral(i) => Ok(i.to_string()),
        ASTNode::TxtLiteral(s) => Ok(s),
        _ => Err("Unsupported expression.".to_string()),
    }
}
