mod expr_string;
mod format_step;
mod format_steps;
mod mathexpr;

pub use mathexpr::MathExpr;

use crate::builtins::math::{parser::ExprNode, transformer::TransformStep};
use expr_string::expr_to_string;
use format_steps::format_steps;

pub fn render_equation(
    original: &str,
    variable: &str,
    transform_steps: &[TransformStep],
    final_expr: &ExprNode,
) -> MathExpr {
    let steps = format_steps(transform_steps);
    let result = expr_to_string(final_expr);
    MathExpr::new(original.to_string(), variable.to_string(), steps, result)
}
