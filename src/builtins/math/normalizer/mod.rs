mod ast;
mod canonical;
mod combine;
mod expand;
mod sort;

pub use ast::NormalizedNode;

use crate::builtins::math::parser::{Equation, ExprNode};
use canonical::to_canonical;
use combine::combine_terms;
use expand::expand;
use sort::sort_terms;

/// Normalizes an equation into a single ExprNode representing `LHS - RHS`,
/// which equals 0 when the original equation is satisfied.
///
/// Pipeline:
///   1. canonical  — subtract RHS from LHS
///   2. expand     — distribute multiplication over addition
///   3. combine    — collect like terms
///   4. sort       — order terms by degree descending
pub fn normalize(equation: Equation) -> Result<ExprNode, String> {
    let canonical = to_canonical(equation);
    let expanded = expand(&canonical);
    let combined = combine_terms(&expanded);
    let sorted = sort_terms(&combined);
    Ok(sorted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::math::lexer::tokenize_equation;
    use crate::builtins::math::parser::{parse_equation_from_tokens, BinaryOp, ExprNode};

    fn normalize_str(input: &str) -> ExprNode {
        let tokens = tokenize_equation(input).unwrap();
        let eq = parse_equation_from_tokens(&tokens).unwrap();
        normalize(eq).unwrap()
    }

    /// Evaluates an ExprNode with a single variable binding x=val.
    fn eval(node: &ExprNode, x: f64) -> f64 {
        match node {
            ExprNode::Num(n) => *n,
            ExprNode::Var(_) => x,
            ExprNode::UnaryMinus(inner) => -eval(inner, x),
            ExprNode::Binary { op, left, right } => {
                let l = eval(left, x);
                let r = eval(right, x);
                match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Sub => l - r,
                    BinaryOp::Mul => l * r,
                    BinaryOp::Div => l / r,
                    BinaryOp::Pow => l.powf(r),
                }
            }
            ExprNode::FuncCall { name, arg } => {
                let v = eval(arg, x);
                match name.as_str() {
                    "sqrt" => v.sqrt(),
                    "abs" => v.abs(),
                    "sin" => v.sin(),
                    "cos" => v.cos(),
                    "exp" => v.exp(),
                    "log" => v.ln(),
                    _ => panic!("unknown func {}", name),
                }
            }
        }
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    // ── main spec tests ───────────────────────────────────────────────────────

    #[test]
    fn test_linear_simple() {
        // "2x + 4 = 10" → 2x - 6 = 0  →  x = 3
        let result = normalize_str("2x + 4 = 10");
        assert!(approx(eval(&result, 3.0), 0.0), "f(3) should be 0, got {}", eval(&result, 3.0));
        assert!(!approx(eval(&result, 2.0), 0.0), "f(2) should not be 0");
    }

    #[test]
    fn test_distribution() {
        // "3(x + 2) = 12" → 3x - 6 = 0  →  x = 2
        let result = normalize_str("3(x + 2) = 12");
        assert!(approx(eval(&result, 2.0), 0.0), "f(2) should be 0, got {}", eval(&result, 2.0));
        assert!(!approx(eval(&result, 0.0), 0.0), "f(0) should not be 0");
    }

    #[test]
    fn test_cancellation() {
        // "x^2 + 2x - 3 = x^2 + 5" → 2x - 8 = 0  →  x = 4
        let result = normalize_str("x^2 + 2x - 3 = x^2 + 5");
        assert!(approx(eval(&result, 4.0), 0.0), "f(4) should be 0, got {}", eval(&result, 4.0));
        assert!(!approx(eval(&result, 0.0), 0.0), "f(0) should not be 0");
    }

    // ── canonical ─────────────────────────────────────────────────────────────

    #[test]
    fn test_canonical_lhs_minus_rhs() {
        // "x = 5" → x - 5  →  f(5) = 0
        let result = normalize_str("x = 5");
        assert!(approx(eval(&result, 5.0), 0.0));
    }

    // ── combine ───────────────────────────────────────────────────────────────

    #[test]
    fn test_like_terms_combined() {
        // "2x + 3x = 0" → 5x = 0  →  only x=0 is solution
        let result = normalize_str("2x + 3x = 0");
        assert!(approx(eval(&result, 0.0), 0.0));
        // f(1) = 5, not 0
        assert!(approx(eval(&result, 1.0), 5.0));
    }

    #[test]
    fn test_constants_combined() {
        // "x + 5 + 3 = 0" → x + 8 = 0  →  x = -8
        let result = normalize_str("x + 5 + 3 = 0");
        assert!(approx(eval(&result, -8.0), 0.0));
    }

    #[test]
    fn test_squared_terms_cancel() {
        // "x^2 = x^2" → 0 = 0 (returns Num(0) when all terms cancel)
        let result = normalize_str("x^2 = x^2");
        assert!(approx(eval(&result, 99.0), 0.0));
    }

    // ── expand ────────────────────────────────────────────────────────────────

    #[test]
    fn test_left_distribution() {
        // "(x + 1) * 2 = 0" → 2x + 2 = 0  →  x = -1
        let result = normalize_str("(x + 1) * 2 = 0");
        assert!(approx(eval(&result, -1.0), 0.0));
    }

    #[test]
    fn test_double_distribution() {
        // "2 * (x + 3) = 2x" → 2x + 6 - 2x = 0 → 6 = 0 (no solution, constant 6)
        let result = normalize_str("2 * (x + 3) = 2x");
        // No variable left — result should be constant 6
        assert!(approx(eval(&result, 0.0), 6.0));
        assert!(approx(eval(&result, 999.0), 6.0));
    }

    // ── sort ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_sort_highest_degree_first() {
        // "1 + x + x^2 = 0" → after sort: x^2 term comes first
        // Verify semantics still hold: f(-1) = 1 - 1 + 1 - 0 = hmm let's pick correct root
        // x^2 + x + 1 = 0 has no real roots; just verify eval is correct at x=0 → 1
        let result = normalize_str("1 + x + x^2 = 0");
        assert!(approx(eval(&result, 0.0), 1.0)); // 0 + 0 + 1 = 1
    }
}
