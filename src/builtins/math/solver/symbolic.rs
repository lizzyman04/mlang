use crate::builtins::math::parser::{BinaryOp, ExprNode};

fn apply_inverse(func_name: &str, rhs: ExprNode) -> ExprNode {
    match func_name {
        "sqrt" => ExprNode::Binary {
            op: BinaryOp::Pow,
            left: Box::new(rhs),
            right: Box::new(ExprNode::Num(2.0)),
        },
        "log" => ExprNode::Binary {
            op: BinaryOp::Pow,
            left: Box::new(ExprNode::Num(std::f64::consts::E)),
            right: Box::new(rhs),
        },
        "exp" => ExprNode::FuncCall {
            name: "log".to_string(),
            arg: Box::new(rhs),
        },
        _ => rhs,
    }
}

/// Solves `FuncCall(var) = rhs` by applying the inverse function to rhs.
///
/// Handles the normalized form `FuncCall(var) - rhs = 0`.
/// Returns the symbolic or numeric result for `var`.
pub fn preserve_symbolic(expr: &ExprNode, var: &str) -> ExprNode {
    if let ExprNode::Binary { op: BinaryOp::Sub, left, right } = expr {
        if let ExprNode::FuncCall { name, arg } = left.as_ref() {
            if let ExprNode::Var(v) = arg.as_ref() {
                if v == var {
                    return apply_inverse(name, *right.clone());
                }
            }
        }
    }
    expr.clone()
}
