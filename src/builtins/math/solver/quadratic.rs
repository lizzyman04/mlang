use crate::builtins::math::parser::{BinaryOp, ExprNode};

type Term = (f64, Option<ExprNode>);

fn flatten(node: &ExprNode, sign: f64) -> Vec<Term> {
    match node {
        ExprNode::Num(n) => vec![(sign * n, None)],
        ExprNode::Binary { op: BinaryOp::Add, left, right } => {
            let mut v = flatten(left, sign);
            v.extend(flatten(right, sign));
            v
        }
        ExprNode::Binary { op: BinaryOp::Sub, left, right } => {
            let mut v = flatten(left, sign);
            v.extend(flatten(right, -sign));
            v
        }
        ExprNode::Binary { op: BinaryOp::Mul, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (ExprNode::Num(c), base) => vec![(sign * c, Some(base.clone()))],
                (base, ExprNode::Num(c)) => vec![(sign * c, Some(base.clone()))],
                _ => vec![(sign, Some(node.clone()))],
            }
        }
        ExprNode::UnaryMinus(inner) => flatten(inner, -sign),
        _ => vec![(sign, Some(node.clone()))],
    }
}

fn term_degree(base: &ExprNode, var: &str) -> u32 {
    match base {
        ExprNode::Var(name) if name == var => 1,
        ExprNode::Binary { op: BinaryOp::Pow, left, right } => {
            if let ExprNode::Var(v) = left.as_ref() {
                if v == var {
                    if let ExprNode::Num(n) = right.as_ref() {
                        return *n as u32;
                    }
                }
            }
            0
        }
        _ => 0,
    }
}

pub fn solve_quadratic(expr: &ExprNode, var: &str) -> Result<Vec<ExprNode>, String> {
    let terms = flatten(expr, 1.0);

    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;

    for (coeff, base) in terms {
        match base {
            None => c += coeff,
            Some(base_expr) => match term_degree(&base_expr, var) {
                2 => a += coeff,
                1 => b += coeff,
                _ => return Err(format!("unsupported term in quadratic: {:?}", base_expr)),
            },
        }
    }

    if a.abs() < 1e-12 {
        return Err("leading coefficient is zero — not a quadratic equation".to_string());
    }

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < -1e-12 {
        return Err("no real solution".to_string());
    }

    if discriminant.abs() < 1e-12 {
        return Ok(vec![ExprNode::Num(-b / (2.0 * a))]);
    }

    let sqrt_d = discriminant.sqrt();
    Ok(vec![
        ExprNode::Num((-b + sqrt_d) / (2.0 * a)),
        ExprNode::Num((-b - sqrt_d) / (2.0 * a)),
    ])
}
