use super::ast::BinaryOp;

/// Returns the binding power of a binary operator.
/// Higher value = tighter binding.
///
/// Add/Sub: 1  (lowest)
/// Mul/Div: 2
/// Pow:     3  (highest, right-associative)
pub fn get_precedence(op: &BinaryOp) -> u8 {
    match op {
        BinaryOp::Add | BinaryOp::Sub => 1,
        BinaryOp::Mul | BinaryOp::Div => 2,
        BinaryOp::Pow => 3,
    }
}

pub fn is_right_associative(op: &BinaryOp) -> bool {
    matches!(op, BinaryOp::Pow)
}
