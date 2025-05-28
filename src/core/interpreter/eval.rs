use crate::core::parser::ast::Expression;
use super::env::Environment;

pub fn eval_expression(
    expr: Expression,
    env: &Environment,
) -> Result<String, String> {
    match expr {
        Expression::IntLiteral(i) => Ok(i.to_string()),
        Expression::DecLiteral(f) => Ok(f.to_string()),
        Expression::TxtLiteral(s) => Ok(s),
        Expression::Identifier(name) => {
            if let Some((_typ, value)) = env.get(&name) {
                eval_expression(value.clone(), env)
            } else {
                Err(format!("Undefined variable '{}'", name))
            }
        }
    }
}
