use crate::builtins::math::parser::{BinaryOp, ExprNode};

/// Distributes multiplication over addition/subtraction recursively until no
/// more distribution is possible.
pub fn expand(node: &ExprNode) -> ExprNode {
    match node {
        ExprNode::Binary { op: BinaryOp::Mul, left, right } => {
            distribute(expand(left), expand(right))
        }
        ExprNode::Binary { op, left, right } => ExprNode::Binary {
            op: op.clone(),
            left: Box::new(expand(left)),
            right: Box::new(expand(right)),
        },
        ExprNode::UnaryMinus(inner) => ExprNode::UnaryMinus(Box::new(expand(inner))),
        _ => node.clone(),
    }
}

fn distribute(left: ExprNode, right: ExprNode) -> ExprNode {
    match (&left, &right) {
        // a * (b + c) → a*b + a*c
        (_, ExprNode::Binary { op: BinaryOp::Add, left: b, right: c }) => ExprNode::Binary {
            op: BinaryOp::Add,
            left: Box::new(distribute(left.clone(), *b.clone())),
            right: Box::new(distribute(left, *c.clone())),
        },
        // a * (b - c) → a*b - a*c
        (_, ExprNode::Binary { op: BinaryOp::Sub, left: b, right: c }) => ExprNode::Binary {
            op: BinaryOp::Sub,
            left: Box::new(distribute(left.clone(), *b.clone())),
            right: Box::new(distribute(left, *c.clone())),
        },
        // (a + b) * c → a*c + b*c
        (ExprNode::Binary { op: BinaryOp::Add, left: a, right: b }, _) => ExprNode::Binary {
            op: BinaryOp::Add,
            left: Box::new(distribute(*a.clone(), right.clone())),
            right: Box::new(distribute(*b.clone(), right)),
        },
        // (a - b) * c → a*c - b*c
        (ExprNode::Binary { op: BinaryOp::Sub, left: a, right: b }, _) => ExprNode::Binary {
            op: BinaryOp::Sub,
            left: Box::new(distribute(*a.clone(), right.clone())),
            right: Box::new(distribute(*b.clone(), right)),
        },
        // Nothing to distribute
        _ => ExprNode::Binary {
            op: BinaryOp::Mul,
            left: Box::new(left),
            right: Box::new(right),
        },
    }
}
