use super::env::Environment;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::Expression;

pub fn eval_expression(expr: Expression, env: &Environment) -> Result<Expression, String> {
    match expr {
        Expression::IntLiteral(_)
        | Expression::DecLiteral(_)
        | Expression::BoolLiteral(_)
        | Expression::TxtLiteral(_) => Ok(expr),

        Expression::Identifier(name) => {
            if let Some((_typ, value)) = env.get(&name) {
                eval_expression(value.clone(), env)
            } else {
                Err(format!("Undefined variable '{}'", name))
            }
        }

        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let left_eval = eval_expression(*left, env)?;
            let right_eval = eval_expression(*right, env)?;

            match (operator, left_eval, right_eval) {
                (TokenKind::Plus, Expression::IntLiteral(a), Expression::IntLiteral(b)) => {
                    Ok(Expression::IntLiteral(a + b))
                }
                (TokenKind::Plus, Expression::DecLiteral(a), Expression::DecLiteral(b)) => {
                    Ok(Expression::DecLiteral(a + b))
                }
                (TokenKind::Plus, Expression::TxtLiteral(a), Expression::TxtLiteral(b)) => {
                    Ok(Expression::TxtLiteral(a + &b))
                }

                _ => Err("Unsupported binary operation or type mismatch.".to_string()),
            }
        }
    }
}
