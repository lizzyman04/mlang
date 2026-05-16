use crate::builtins::math::parser::ExprNode;
use super::expr_string::expr_to_string;

pub fn format_step(expr: &ExprNode, rule: &str, step_num: usize) -> String {
    format!("Step {}: {}  # {}", step_num, expr_to_string(expr), rule)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::math::parser::ExprNode;

    #[test]
    fn test_format_step_contains_num_expr_rule() {
        let expr = ExprNode::Var("x".to_string());
        let result = format_step(&expr, "initial", 1);
        assert_eq!(result, "Step 1: x  # initial");
    }

    #[test]
    fn test_format_step_zero_indexed() {
        let expr = ExprNode::Num(42.0);
        let result = format_step(&expr, "solve", 0);
        assert_eq!(result, "Step 0: 42  # solve");
    }
}
