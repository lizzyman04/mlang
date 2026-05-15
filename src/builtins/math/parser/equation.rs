use super::ast::Equation;
use super::expr::parse_expr;
use crate::builtins::math::lexer::MathToken;

pub fn parse_equation(tokens: &[MathToken]) -> Result<Equation, String> {
    let eq_pos = tokens
        .iter()
        .position(|t| t == &MathToken::Eq)
        .ok_or_else(|| "no '=' found in equation".to_string())?;

    let lhs_tokens = &tokens[..eq_pos];
    let rhs_tokens = &tokens[eq_pos + 1..];

    if lhs_tokens.is_empty() {
        return Err("empty left-hand side".to_string());
    }
    if rhs_tokens.is_empty() {
        return Err("empty right-hand side".to_string());
    }

    let mut pos = 0;
    let lhs = parse_expr(lhs_tokens, &mut pos, 0)?;
    if pos != lhs_tokens.len() {
        return Err(format!(
            "unexpected token on left side: {:?}",
            lhs_tokens[pos]
        ));
    }

    pos = 0;
    let rhs = parse_expr(rhs_tokens, &mut pos, 0)?;
    if pos != rhs_tokens.len() {
        return Err(format!(
            "unexpected token on right side: {:?}",
            rhs_tokens[pos]
        ));
    }

    Ok(Equation { lhs: Box::new(lhs), rhs: Box::new(rhs) })
}
