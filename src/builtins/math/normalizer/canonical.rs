use crate::builtins::math::parser::{BinaryOp, Equation, ExprNode};

/// Converts an equation into `LHS - RHS` form, yielding a single ExprNode
/// that equals 0 when the equation is satisfied.
pub fn to_canonical(equation: Equation) -> ExprNode {
    ExprNode::Binary {
        op: BinaryOp::Sub,
        left: equation.lhs,
        right: equation.rhs,
    }
}
