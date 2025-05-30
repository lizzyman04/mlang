use crate::core::interpreter::env::Environment;
use crate::core::lexer::symbol::simple::SimpleSymbolKind;
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

            match (left_val, right_val, &operator.kind) {
                (IntLiteral(a), IntLiteral(b), TokenKind::SimpleSymbol(SimpleSymbolKind::Plus)) => {
                    Ok(IntLiteral(a + b))
                }
                (
                    IntLiteral(a),
                    IntLiteral(b),
                    TokenKind::SimpleSymbol(SimpleSymbolKind::Minus),
                ) => Ok(IntLiteral(a - b)),
                (IntLiteral(a), IntLiteral(b), TokenKind::SimpleSymbol(SimpleSymbolKind::Star)) => {
                    Ok(IntLiteral(a * b))
                }
                (
                    IntLiteral(a),
                    IntLiteral(b),
                    TokenKind::SimpleSymbol(SimpleSymbolKind::Slash),
                ) => {
                    if b == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(IntLiteral(a / b))
                    }
                }

                (DecLiteral(a), DecLiteral(b), TokenKind::SimpleSymbol(SimpleSymbolKind::Plus)) => {
                    Ok(DecLiteral(a + b))
                }
                (
                    DecLiteral(a),
                    DecLiteral(b),
                    TokenKind::SimpleSymbol(SimpleSymbolKind::Minus),
                ) => Ok(DecLiteral(a - b)),
                (DecLiteral(a), DecLiteral(b), TokenKind::SimpleSymbol(SimpleSymbolKind::Star)) => {
                    Ok(DecLiteral(a * b))
                }
                (
                    DecLiteral(a),
                    DecLiteral(b),
                    TokenKind::SimpleSymbol(SimpleSymbolKind::Slash),
                ) => {
                    if b == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(DecLiteral(a / b))
                    }
                }

                (_, _, TokenKind::MathSymbol(kind)) => {
                    return Err(format!(
                        "Unsupported math symbol '{}' in expression at line {}, column {}. Consider using `math(...)` instead.",
                        kind.to_char(),
                        operator.line,
                        operator.column
                    ));
                }
                _ => Err(format!(
                    "Invalid binary operation or type mismatch in expression. (line {})",
                    operator.line
                )),
            }
        }
    }
}
