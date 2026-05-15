use crate::builtins::math::parser::{BinaryOp, ExprNode};

/// Evaluates a node that contains only numeric literals and operations.
/// Returns None if the node contains any variable or function call.
fn eval_numeric(node: &ExprNode) -> Option<f64> {
    match node {
        ExprNode::Num(n) => Some(*n),
        ExprNode::UnaryMinus(inner) => eval_numeric(inner).map(|n| -n),
        ExprNode::Binary { op, left, right } => {
            let l = eval_numeric(left)?;
            let r = eval_numeric(right)?;
            match op {
                BinaryOp::Add => Some(l + r),
                BinaryOp::Sub => Some(l - r),
                BinaryOp::Mul => Some(l * r),
                BinaryOp::Div if r != 0.0 => Some(l / r),
                BinaryOp::Pow => Some(l.powf(r)),
                _ => None,
            }
        }
        _ => None,
    }
}

/// A single term: (coefficient, optional symbolic base).
/// (3.0, Some(Var("x"))) represents `3x`.
/// (−6.0, None)          represents the constant `−6`.
type Term = (f64, Option<ExprNode>);

/// Flattens Add/Sub tree into a flat list of signed terms.
fn flatten(node: &ExprNode, sign: f64) -> Vec<Term> {
    // Pure numeric subexpression → single constant term
    if let Some(n) = eval_numeric(node) {
        return vec![(sign * n, None)];
    }

    match node {
        ExprNode::Binary { op: BinaryOp::Add, left, right } => {
            let mut terms = flatten(left, sign);
            terms.extend(flatten(right, sign));
            terms
        }
        ExprNode::Binary { op: BinaryOp::Sub, left, right } => {
            let mut terms = flatten(left, sign);
            terms.extend(flatten(right, -sign));
            terms
        }
        ExprNode::Binary { op: BinaryOp::Mul, left, right } => {
            // Extract a leading numeric coefficient if present
            match (left.as_ref(), right.as_ref()) {
                (ExprNode::Num(c), base) => vec![(sign * c, Some(base.clone()))],
                (base, ExprNode::Num(c)) => vec![(sign * c, Some(base.clone()))],
                _ => vec![(sign, Some(node.clone()))],
            }
        }
        ExprNode::UnaryMinus(inner) => flatten(inner, -sign),
        ExprNode::Var(_) => vec![(sign, Some(node.clone()))],
        _ => vec![(sign, Some(node.clone()))],
    }
}

/// Groups terms with structurally equal bases and sums their coefficients.
fn group(terms: Vec<Term>) -> Vec<Term> {
    let mut combined: Vec<Term> = Vec::new();

    'outer: for (coeff, base) in terms {
        for (existing_coeff, existing_base) in combined.iter_mut() {
            if *existing_base == base {
                *existing_coeff += coeff;
                continue 'outer;
            }
        }
        combined.push((coeff, base));
    }

    // Drop zero-coefficient terms
    combined.retain(|(c, _)| c.abs() > 1e-12);
    combined
}

/// Rebuilds a sorted list of terms into a left-leaning Add tree.
fn rebuild(terms: Vec<Term>) -> ExprNode {
    if terms.is_empty() {
        return ExprNode::Num(0.0);
    }

    let mut iter = terms.into_iter();
    let mut acc = term_to_node(iter.next().unwrap());
    for t in iter {
        acc = ExprNode::Binary {
            op: BinaryOp::Add,
            left: Box::new(acc),
            right: Box::new(term_to_node(t)),
        };
    }
    acc
}

fn term_to_node((coeff, base): Term) -> ExprNode {
    match base {
        None => ExprNode::Num(coeff),
        Some(base_expr) => {
            if (coeff - 1.0).abs() < 1e-12 {
                base_expr
            } else if (coeff + 1.0).abs() < 1e-12 {
                ExprNode::UnaryMinus(Box::new(base_expr))
            } else {
                ExprNode::Binary {
                    op: BinaryOp::Mul,
                    left: Box::new(ExprNode::Num(coeff)),
                    right: Box::new(base_expr),
                }
            }
        }
    }
}

/// Collects like terms in an expression tree.
/// `2x + 3x → 5x`, `x^2 - x^2 → 0 (dropped)`, `5 + 3 → 8`.
pub fn combine_terms(node: &ExprNode) -> ExprNode {
    let terms = flatten(node, 1.0);
    let grouped = group(terms);
    rebuild(grouped)
}
