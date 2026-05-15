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

fn extract_coeff(node: &ExprNode, var: &str) -> Result<f64, String> {
    match node {
        ExprNode::Var(name) if name == var => Ok(1.0),
        ExprNode::Binary { op: BinaryOp::Mul, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (ExprNode::Num(c), ExprNode::Var(v)) if v == var => Ok(*c),
                (ExprNode::Var(v), ExprNode::Num(c)) if v == var => Ok(*c),
                (ExprNode::Num(c), inner) => Ok(c * extract_coeff(inner, var)?),
                (inner, ExprNode::Num(c)) => Ok(c * extract_coeff(inner, var)?),
                _ => Err(format!("cannot extract coefficient of '{}' from Mul", var)),
            }
        }
        ExprNode::Binary { op: BinaryOp::Div, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (ExprNode::Var(v), ExprNode::Num(n)) if v == var && *n != 0.0 => Ok(1.0 / n),
                _ => Err(format!("cannot extract coefficient of '{}' from Div", var)),
            }
        }
        other => Err(format!("cannot extract coefficient of '{}' from {:?}", var, other)),
    }
}

pub fn solve_linear(expr: &ExprNode, var: &str) -> Result<ExprNode, String> {
    let terms = flatten(expr, 1.0);

    let mut var_terms: Vec<(f64, ExprNode)> = Vec::new();
    let mut const_sum: f64 = 0.0;

    for (coeff, base) in terms {
        match base {
            None => const_sum += coeff,
            Some(base_expr) => var_terms.push((coeff, base_expr)),
        }
    }

    if var_terms.is_empty() {
        return Err("no variable terms found".to_string());
    }

    let mut effective_coeff: f64 = 0.0;
    for (outer, base) in &var_terms {
        let inner = extract_coeff(base, var).map_err(|e| format!("unsupported term: {}", e))?;
        effective_coeff += outer * inner;
    }

    if effective_coeff.abs() < 1e-12 {
        return Err(format!(
            "coefficient of '{}' is zero — equation has no unique solution",
            var
        ));
    }

    Ok(ExprNode::Num(-const_sum / effective_coeff))
}
