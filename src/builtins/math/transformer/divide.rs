use crate::builtins::math::parser::{BinaryOp, ExprNode};

/// Extracts the effective numeric coefficient of `var` from a single term node.
///
/// | node            | result |
/// |-----------------|--------|
/// | `Var("x")`      | 1.0    |
/// | `Mul(c, Var)`   | c      |
/// | `Mul(Var, c)`   | c      |
/// | `Div(Var, n)`   | 1/n    |
/// | `Mul(c, inner)` | c * coeff(inner) |
pub fn divide_coefficient(node: &ExprNode, var: &str) -> Result<f64, String> {
    match node {
        ExprNode::Var(name) if name == var => Ok(1.0),

        ExprNode::Binary { op: BinaryOp::Mul, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (ExprNode::Num(c), ExprNode::Var(v)) if v == var => Ok(*c),
                (ExprNode::Var(v), ExprNode::Num(c)) if v == var => Ok(*c),
                (ExprNode::Num(c), inner) => Ok(c * divide_coefficient(inner, var)?),
                (inner, ExprNode::Num(c)) => Ok(c * divide_coefficient(inner, var)?),
                _ => Err(format!("cannot extract coefficient of '{}' from Mul node", var)),
            }
        }

        ExprNode::Binary { op: BinaryOp::Div, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (ExprNode::Var(v), ExprNode::Num(n)) if v == var && *n != 0.0 => Ok(1.0 / n),
                _ => Err(format!("cannot extract coefficient of '{}' from Div node", var)),
            }
        }

        other => Err(format!(
            "cannot extract coefficient of '{}' from {:?}",
            var, other
        )),
    }
}
