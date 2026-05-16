use crate::builtins::math::lexer::tokenize_equation;
use crate::builtins::math::normalizer::normalize;
use crate::builtins::math::parser::{parse_equation_from_tokens, BinaryOp, ExprNode};
use crate::builtins::math::renderer::{render_equation, MathExpr};
use crate::builtins::math::solver::solve;
use crate::builtins::math::transformer::{render_expr, transform};

/// Full pipeline: tokenize → parse → normalize → transform → render.
/// Falls back to the algebraic solver (no steps) for non-linear equations.
pub fn solve_equation(eq_str: &str) -> Result<MathExpr, String> {
    let tokens = tokenize_equation(eq_str)?;
    let equation = parse_equation_from_tokens(&tokens)?;
    let normalized = normalize(equation)?;
    let var = detect_variable(&normalized)?;

    if let Ok((final_expr, transform_steps)) = transform(&normalized) {
        return Ok(render_equation(eq_str, &var, &transform_steps, &final_expr));
    }

    let solutions = solve(&normalized).map_err(|e| e.to_string())?;
    let result = format_solutions(&var, &solutions);
    Ok(MathExpr::new(eq_str.to_string(), var, vec![], result))
}

fn format_solutions(var: &str, solutions: &[ExprNode]) -> String {
    match solutions {
        [] => "no solution".to_string(),
        [single] => format!("{} = {}", var, render_expr(single)),
        multiple => multiple
            .iter()
            .map(|s| format!("{} = {}", var, render_expr(s)))
            .collect::<Vec<_>>()
            .join(", "),
    }
}

/// Simplifies an expression (no `=` required) by combining like terms.
/// Appends `= 0` internally so the equation parser accepts it.
pub fn simplify_expression(expr_str: &str) -> Result<String, String> {
    let eq_str = format!("{} = 0", expr_str);
    let tokens = tokenize_equation(&eq_str)?;
    let equation = parse_equation_from_tokens(&tokens)?;
    let normalized = normalize(equation)?;
    Ok(render_expr(&normalized))
}

/// Numerically evaluates a constant expression (no variables).
/// Supports lexer constants: pi, e, tau, inf.
pub fn evaluate_expression(expr_str: &str) -> Result<f64, String> {
    let eq_str = format!("{} = 0", expr_str);
    let tokens = tokenize_equation(&eq_str)?;
    let equation = parse_equation_from_tokens(&tokens)?;
    let normalized = normalize(equation)?;
    eval_numeric(&normalized)
        .ok_or_else(|| "expression contains free variables or unsupported operations".to_string())
}

fn eval_numeric(node: &ExprNode) -> Option<f64> {
    match node {
        ExprNode::Num(n) => Some(*n),
        ExprNode::UnaryMinus(inner) => eval_numeric(inner).map(|n| -n),
        ExprNode::Binary { op, left, right } => {
            let l = eval_numeric(left)?;
            let r = eval_numeric(right)?;
            match op {
                BinaryOp::Add => Some(l + r),
                BinaryOp::Sub => Some(l - r),
                BinaryOp::Mul => Some(l * r),
                BinaryOp::Div if r != 0.0 => Some(l / r),
                BinaryOp::Pow => Some(l.powf(r)),
                _ => None,
            }
        }
        ExprNode::FuncCall { name, arg } => {
            let v = eval_numeric(arg)?;
            match name.as_str() {
                "sqrt" => Some(v.sqrt()),
                "abs" => Some(v.abs()),
                "sin" => Some(v.sin()),
                "cos" => Some(v.cos()),
                "tan" => Some(v.tan()),
                "exp" => Some(v.exp()),
                "log" => Some(v.ln()),
                _ => None,
            }
        }
        _ => None,
    }
}

fn detect_variable(node: &ExprNode) -> Result<String, String> {
    fn first_var(node: &ExprNode) -> Option<String> {
        match node {
            ExprNode::Var(name) => Some(name.clone()),
            ExprNode::Binary { left, right, .. } => {
                first_var(left).or_else(|| first_var(right))
            }
            ExprNode::UnaryMinus(inner) => first_var(inner),
            ExprNode::FuncCall { arg, .. } => first_var(arg),
            _ => None,
        }
    }
    first_var(node).ok_or_else(|| "no variable found in expression".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_solve_linear() {
        let m = solve_equation("2x + 4 = 10").unwrap();
        assert_eq!(m.result(), "x = 3");
        assert_eq!(m.original(), "2x + 4 = 10");
        assert_eq!(m.variable(), "x");
    }

    #[test]
    fn test_solve_steps_present() {
        let m = solve_equation("3x - 12 = 0").unwrap();
        assert!(!m.steps().is_empty());
        assert!(m.step(0).is_some());
    }

    #[test]
    fn test_solve_error_no_variable() {
        assert!(solve_equation("2 + 4 = 6").is_err());
    }

    #[test]
    fn test_simplify_like_terms() {
        let result = simplify_expression("2x + x + 4 - 1").unwrap();
        assert_eq!(result, "3x + 3");
    }

    #[test]
    fn test_simplify_constants_only() {
        let result = simplify_expression("4 + 3").unwrap();
        assert_eq!(result, "7");
    }

    #[test]
    fn test_evaluate_pi_expr() {
        let result = evaluate_expression("pi * 3^2").unwrap();
        assert!(approx(result, std::f64::consts::PI * 9.0));
    }

    #[test]
    fn test_evaluate_simple() {
        let result = evaluate_expression("2 + 3 * 4").unwrap();
        assert!(approx(result, 14.0));
    }

    #[test]
    fn test_evaluate_error_with_variable() {
        assert!(evaluate_expression("x + 1").is_err());
    }
}
