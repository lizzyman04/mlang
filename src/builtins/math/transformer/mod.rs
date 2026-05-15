mod divide;
mod error;
mod isolate;
mod step;

pub use error::TransformerError;
pub use step::{render_expr, TransformStep};

use crate::builtins::math::parser::ExprNode;
use isolate::{collect_vars, isolate_variable};

/// Transforms a normalized expression (`= 0`) into a solved variable and step list.
///
/// The input must be the output of `normalizer::normalize()` — a single ExprNode
/// representing `LHS - RHS` that equals 0 at the solution.
///
/// Returns `(solution: ExprNode, steps: Vec<TransformStep>)` where `solution` is
/// the numeric value the variable takes (e.g., `ExprNode::Num(3.0)`).
///
/// Errors:
/// - Multiple variables found
/// - No variable found
/// - Non-linear structure not supported
pub fn transform(equation: &ExprNode) -> Result<(ExprNode, Vec<TransformStep>), String> {
    let vars = collect_vars(equation);

    if vars.is_empty() {
        return Err("no variable found in expression".to_string());
    }
    if vars.len() > 1 {
        let mut names: Vec<_> = vars.into_iter().collect();
        names.sort();
        return Err(format!("multiple variables found: {}", names.join(", ")));
    }

    let target_var = vars.into_iter().next().unwrap();
    isolate_variable(equation, &target_var)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::math::{
        lexer::tokenize_equation,
        normalizer::normalize,
        parser::parse_equation_from_tokens,
    };

    fn run(input: &str) -> (ExprNode, Vec<TransformStep>) {
        let tokens = tokenize_equation(input).unwrap();
        let eq = parse_equation_from_tokens(&tokens).unwrap();
        let normalized = normalize(eq).unwrap();
        transform(&normalized).unwrap()
    }

    fn run_err(input: &str) -> String {
        let tokens = tokenize_equation(input).unwrap();
        let eq = parse_equation_from_tokens(&tokens).unwrap();
        let normalized = normalize(eq).unwrap();
        transform(&normalized).unwrap_err()
    }

    fn solution(input: &str) -> f64 {
        match run(input).0 {
            ExprNode::Num(n) => n,
            other => panic!("expected Num solution, got {:?}", other),
        }
    }

    fn steps(input: &str) -> Vec<String> {
        run(input).1.into_iter().map(|s| s.expression).collect()
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    // ── solution correctness ──────────────────────────────────────────────────

    #[test]
    fn test_linear_with_constants() {
        // "2x + 4 - 10 = 0" → 2x - 6 = 0 → x = 3
        assert!(approx(solution("2x + 4 - 10 = 0"), 3.0));
    }

    #[test]
    fn test_simple_linear() {
        // "3x - 12 = 0" → x = 4
        assert!(approx(solution("3x - 12 = 0"), 4.0));
    }

    #[test]
    fn test_division_term() {
        // "x/2 = 5" → x = 10
        assert!(approx(solution("x/2 = 5"), 10.0));
    }

    #[test]
    fn test_fraction_solution() {
        // "2x = 5" → x = 2.5
        assert!(approx(solution("2x = 5"), 2.5));
    }

    #[test]
    fn test_negative_solution() {
        // "x + 8 = 0" → x = -8
        assert!(approx(solution("x + 8 = 0"), -8.0));
    }

    #[test]
    fn test_coefficient_one() {
        // "x = 5" → x = 5
        assert!(approx(solution("x = 5"), 5.0));
    }

    #[test]
    fn test_with_normalizer_pipeline() {
        // "2x + 4 = 10" full pipeline → x = 3
        assert!(approx(solution("2x + 4 = 10"), 3.0));
    }

    // ── step recording ────────────────────────────────────────────────────────

    #[test]
    fn test_steps_linear() {
        // "3x - 12 = 0" → steps: ["3x - 12 = 0", "3x = 12", "x = 4"]
        let s = steps("3x - 12 = 0");
        assert_eq!(s[0], "3x - 12 = 0");
        assert_eq!(s[1], "3x = 12");
        assert_eq!(s[2], "x = 4");
    }

    #[test]
    fn test_steps_count_simple() {
        // "3x - 12 = 0" → 3 steps
        assert_eq!(run("3x - 12 = 0").1.len(), 3);
    }

    #[test]
    fn test_step_rules() {
        let s = run("3x - 12 = 0").1;
        assert_eq!(s[0].rule, "initial expression");
        assert!(s[1].rule.contains("add") || s[1].rule.contains("subtract"));
        assert!(s[2].rule.contains("divide"));
    }

    #[test]
    fn test_step_nums_sequential() {
        let s = run("2x + 4 = 10").1;
        for (i, step) in s.iter().enumerate() {
            assert_eq!(step.step_num, i);
        }
    }

    // ── error cases ───────────────────────────────────────────────────────────

    #[test]
    fn test_multiple_variables_error() {
        let err = run_err("x + y = 5");
        assert!(err.contains("multiple variables"), "got: {}", err);
    }

    #[test]
    fn test_render_expr_mul() {
        use crate::builtins::math::parser::{BinaryOp, ExprNode};
        let node = ExprNode::Binary {
            op: BinaryOp::Mul,
            left: Box::new(ExprNode::Num(2.0)),
            right: Box::new(ExprNode::Var("x".to_string())),
        };
        assert_eq!(render_expr(&node), "2x");
    }

    #[test]
    fn test_render_add_negative() {
        use crate::builtins::math::parser::{BinaryOp, ExprNode};
        // Add(Var("x"), Num(-6)) → "x - 6"
        let node = ExprNode::Binary {
            op: BinaryOp::Add,
            left: Box::new(ExprNode::Var("x".to_string())),
            right: Box::new(ExprNode::Num(-6.0)),
        };
        assert_eq!(render_expr(&node), "x - 6");
    }
}
