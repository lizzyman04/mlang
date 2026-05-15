mod error;
mod linear;
mod quadratic;
mod symbolic;

pub use error::SolveError;

use std::collections::HashSet;

use crate::builtins::math::parser::{BinaryOp, ExprNode};
use linear::solve_linear;
use quadratic::solve_quadratic;
use symbolic::preserve_symbolic;

fn collect_vars(node: &ExprNode) -> HashSet<String> {
    match node {
        ExprNode::Var(name) => {
            let mut s = HashSet::new();
            s.insert(name.clone());
            s
        }
        ExprNode::Binary { left, right, .. } => {
            let mut s = collect_vars(left);
            s.extend(collect_vars(right));
            s
        }
        ExprNode::UnaryMinus(inner) => collect_vars(inner),
        ExprNode::FuncCall { arg, .. } => collect_vars(arg),
        _ => HashSet::new(),
    }
}

fn contains_var(node: &ExprNode, var: &str) -> bool {
    match node {
        ExprNode::Var(name) => name == var,
        ExprNode::Binary { left, right, .. } => {
            contains_var(left, var) || contains_var(right, var)
        }
        ExprNode::UnaryMinus(inner) => contains_var(inner, var),
        ExprNode::FuncCall { arg, .. } => contains_var(arg, var),
        _ => false,
    }
}

fn contains_func_with_var(node: &ExprNode, var: &str) -> bool {
    match node {
        ExprNode::FuncCall { arg, .. } => contains_var(arg, var),
        ExprNode::Binary { left, right, .. } => {
            contains_func_with_var(left, var) || contains_func_with_var(right, var)
        }
        ExprNode::UnaryMinus(inner) => contains_func_with_var(inner, var),
        _ => false,
    }
}

fn max_degree(node: &ExprNode, var: &str) -> u32 {
    match node {
        ExprNode::Var(name) if name == var => 1,
        ExprNode::Binary { op, left, right } => match op {
            BinaryOp::Pow => {
                if let ExprNode::Var(v) = left.as_ref() {
                    if v == var {
                        if let ExprNode::Num(n) = right.as_ref() {
                            return *n as u32;
                        }
                    }
                }
                0
            }
            BinaryOp::Mul => max_degree(left, var) + max_degree(right, var),
            BinaryOp::Add | BinaryOp::Sub => {
                max_degree(left, var).max(max_degree(right, var))
            }
            BinaryOp::Div => max_degree(left, var),
        },
        ExprNode::UnaryMinus(inner) => max_degree(inner, var),
        ExprNode::FuncCall { arg, .. } => {
            if contains_var(arg, var) { u32::MAX } else { 0 }
        }
        _ => 0,
    }
}

/// Solves a normalized expression (`= 0`) for its single variable.
///
/// Auto-detects the variable. Dispatches to the appropriate solver based on
/// equation degree: 1 → linear, 2 → quadratic, FuncCall → symbolic.
///
/// Returns a `Vec<ExprNode>` since quadratic equations can have two roots.
pub fn solve(expr: &ExprNode) -> Result<Vec<ExprNode>, SolveError> {
    let vars = collect_vars(expr);

    if vars.is_empty() {
        return Err(SolveError::NoVariable("no variable found in expression".to_string()));
    }
    if vars.len() > 1 {
        return Err(SolveError::MultipleVariables(vars));
    }

    let var = vars.into_iter().next().unwrap();

    if contains_func_with_var(expr, &var) {
        let result = preserve_symbolic(expr, &var);
        return Ok(vec![result]);
    }

    match max_degree(expr, &var) {
        1 => {
            let result = solve_linear(expr, &var)
                .map_err(|e| SolveError::Unsupported(e))?;
            Ok(vec![result])
        }
        2 => {
            solve_quadratic(expr, &var).map_err(|e| {
                if e.contains("no real solution") {
                    SolveError::NoSolution(e)
                } else {
                    SolveError::Unsupported(e)
                }
            })
        }
        degree => Err(SolveError::Unsupported(format!(
            "degree-{} equations not supported",
            degree
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::math::{
        lexer::tokenize_equation,
        normalizer::normalize,
        parser::parse_equation_from_tokens,
    };

    fn run(input: &str) -> Vec<ExprNode> {
        let tokens = tokenize_equation(input).unwrap();
        let eq = parse_equation_from_tokens(&tokens).unwrap();
        let normalized = normalize(eq).unwrap();
        solve(&normalized).unwrap()
    }

    fn run_err(input: &str) -> SolveError {
        let tokens = tokenize_equation(input).unwrap();
        let eq = parse_equation_from_tokens(&tokens).unwrap();
        let normalized = normalize(eq).unwrap();
        solve(&normalized).unwrap_err()
    }

    fn nums(input: &str) -> Vec<f64> {
        run(input)
            .into_iter()
            .map(|node| match node {
                ExprNode::Num(n) => n,
                other => panic!("expected Num, got {:?}", other),
            })
            .collect()
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    // ── linear ────────────────────────────────────────────────────────────────

    #[test]
    fn test_linear_positive_solution() {
        // "2x - 6 = 0" → x = 3
        let s = nums("2x - 6 = 0");
        assert_eq!(s.len(), 1);
        assert!(approx(s[0], 3.0));
    }

    #[test]
    fn test_linear_negative_solution() {
        // "3x + 12 = 0" → x = -4
        let s = nums("3x + 12 = 0");
        assert_eq!(s.len(), 1);
        assert!(approx(s[0], -4.0));
    }

    #[test]
    fn test_linear_with_rhs() {
        // "2x + 4 = 10" → x = 3
        let s = nums("2x + 4 = 10");
        assert_eq!(s.len(), 1);
        assert!(approx(s[0], 3.0));
    }

    #[test]
    fn test_linear_division_term() {
        // "x/2 = 5" → x = 10
        let s = nums("x/2 = 5");
        assert_eq!(s.len(), 1);
        assert!(approx(s[0], 10.0));
    }

    // ── quadratic ────────────────────────────────────────────────────────────

    #[test]
    fn test_quadratic_two_roots() {
        // "x^2 - 5x + 6 = 0" → roots 3 and 2
        let s = nums("x^2 - 5x + 6 = 0");
        assert_eq!(s.len(), 2);
        assert!(s.iter().any(|&r| approx(r, 3.0)), "missing root 3");
        assert!(s.iter().any(|&r| approx(r, 2.0)), "missing root 2");
    }

    #[test]
    fn test_quadratic_double_root() {
        // "x^2 + 4x + 4 = 0" → root -2 (double)
        let s = nums("x^2 + 4x + 4 = 0");
        assert_eq!(s.len(), 1);
        assert!(approx(s[0], -2.0));
    }

    #[test]
    fn test_quadratic_no_real_solution() {
        // "x^2 + 1 = 0" → discriminant < 0
        let err = run_err("x^2 + 1 = 0");
        assert!(
            matches!(err, SolveError::NoSolution(_)),
            "expected NoSolution, got {:?}",
            err
        );
    }

    // ── error cases ───────────────────────────────────────────────────────────

    #[test]
    fn test_multiple_variables_error() {
        let err = run_err("x + y = 5");
        assert!(
            matches!(err, SolveError::MultipleVariables(_)),
            "expected MultipleVariables, got {:?}",
            err
        );
    }

    #[test]
    fn test_error_display_multiple_vars() {
        let err = run_err("x + y = 5");
        let msg = err.to_string();
        assert!(msg.contains("multiple variables"), "got: {}", msg);
    }

    // ── solve_linear dispatched from solve() ─────────────────────────────────

    #[test]
    fn test_coefficient_one() {
        // "x = 7" → x = 7
        let s = nums("x = 7");
        assert_eq!(s.len(), 1);
        assert!(approx(s[0], 7.0));
    }
}
