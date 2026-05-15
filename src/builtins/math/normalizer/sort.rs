use crate::builtins::math::parser::{BinaryOp, ExprNode};

/// Returns the polynomial degree of a term node.
fn term_degree(node: &ExprNode) -> i32 {
    match node {
        ExprNode::Num(_) => 0,
        ExprNode::Var(_) => 1,
        ExprNode::UnaryMinus(inner) => term_degree(inner),
        ExprNode::Binary { op: BinaryOp::Pow, right, .. } => match right.as_ref() {
            ExprNode::Num(n) => *n as i32,
            _ => 1,
        },
        ExprNode::Binary { op: BinaryOp::Mul, left, right } => {
            term_degree(left).max(term_degree(right))
        }
        _ => 0,
    }
}

/// Flattens a left-leaning Add chain into a flat Vec of terms.
fn flatten_add(node: ExprNode) -> Vec<ExprNode> {
    match node {
        ExprNode::Binary { op: BinaryOp::Add, left, right } => {
            let mut terms = flatten_add(*left);
            terms.extend(flatten_add(*right));
            terms
        }
        other => vec![other],
    }
}

/// Orders terms by polynomial degree descending (constants last).
pub fn sort_terms(node: &ExprNode) -> ExprNode {
    let mut terms = flatten_add(node.clone());

    // Stable sort so equal-degree terms keep their relative order.
    terms.sort_by(|a, b| term_degree(b).cmp(&term_degree(a)));

    let mut iter = terms.into_iter();
    let mut acc = iter.next().unwrap_or(ExprNode::Num(0.0));
    for term in iter {
        acc = ExprNode::Binary {
            op: BinaryOp::Add,
            left: Box::new(acc),
            right: Box::new(term),
        };
    }
    acc
}
