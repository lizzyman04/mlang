use crate::core::interpreter::env::Environment;
use crate::core::lexer::token::TokenKind;
use crate::core::parser::ast::expr::Expression;
use crate::core::parser::ast::expr::Expression::*;

pub fn evaluate(expr: Expression, env: &Environment) -> Result<Expression, String> {
    match expr {
        IntLiteral(_) | DecLiteral(_) | TxtLiteral(_) | BoolLiteral(_) => Ok(expr),

        Identifier(name) => env
            .get(&name)
            .map(|(_, value)| value.clone())
            .ok_or_else(|| format!("Undefined variable '{}'", name)),

        Binary {
            left,
            operator,
            right,
        } => {
            let left_val = evaluate(*left, env)?;
            let right_val = evaluate(*right, env)?;

            match (left_val, right_val, operator) {
                (IntLiteral(a), IntLiteral(b), TokenKind::Plus) => Ok(IntLiteral(a + b)),
                (IntLiteral(a), IntLiteral(b), TokenKind::Minus) => Ok(IntLiteral(a - b)),
                (IntLiteral(a), IntLiteral(b), TokenKind::Star) => Ok(IntLiteral(a * b)),
                (IntLiteral(a), IntLiteral(b), TokenKind::Slash) => {
                    if b == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(IntLiteral(a / b))
                    }
                }

                (DecLiteral(a), DecLiteral(b), TokenKind::Plus) => Ok(DecLiteral(a + b)),
                (DecLiteral(a), DecLiteral(b), TokenKind::Minus) => Ok(DecLiteral(a - b)),
                (DecLiteral(a), DecLiteral(b), TokenKind::Star) => Ok(DecLiteral(a * b)),
                (DecLiteral(a), DecLiteral(b), TokenKind::Slash) => {
                    if b == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(DecLiteral(a / b))
                    }
                }

                _ => Err("Unsupported binary operation or type mismatch.".to_string()),
            }
        }
    }
}
