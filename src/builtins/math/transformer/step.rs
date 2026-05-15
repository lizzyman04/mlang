use crate::builtins::math::parser::{BinaryOp, ExprNode};

#[derive(Debug, Clone)]
pub struct TransformStep {
    pub step_num: usize,
    pub expression: String,
    pub rule: String,
}

/// Records a step with the full equation as a pre-formatted string.
pub fn record_step(steps: &mut Vec<TransformStep>, expression: String, rule: &str) {
    steps.push(TransformStep {
        step_num: steps.len(),
        expression,
        rule: rule.to_string(),
    });
}

/// Renders an ExprNode as a human-readable math string.
pub fn render_expr(node: &ExprNode) -> String {
    match node {
        ExprNode::Num(n) => render_num(*n),
        ExprNode::Var(name) => name.clone(),
        ExprNode::UnaryMinus(inner) => format!("-{}", render_atom(inner)),
        ExprNode::FuncCall { name, arg } => format!("{}({})", name, render_expr(arg)),
        ExprNode::Binary { op: BinaryOp::Add, left, right } => render_add(left, right),
        ExprNode::Binary { op: BinaryOp::Sub, left, right } => {
            format!("{} - {}", render_expr(left), render_atom(right))
        }
        ExprNode::Binary { op: BinaryOp::Mul, left, right } => render_mul(left, right),
        ExprNode::Binary { op: BinaryOp::Div, left, right } => {
            format!("{} / {}", render_atom(left), render_atom(right))
        }
        ExprNode::Binary { op: BinaryOp::Pow, left, right } => {
            format!("{}^{}", render_atom(left), render_atom(right))
        }
    }
}

fn render_num(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        // Trim trailing zeros for fractions like 0.5, 1.5
        let s = format!("{}", n);
        s
    }
}

/// Wraps compound expressions in parens; atoms are left bare.
fn render_atom(node: &ExprNode) -> String {
    match node {
        ExprNode::Binary { .. } => format!("({})", render_expr(node)),
        ExprNode::UnaryMinus(inner) => format!("(-{})", render_expr(inner)),
        _ => render_expr(node),
    }
}

/// Smart add: `a + Num(-n)` → `"a - n"`, `a + (-b)` → `"a - b"`.
fn render_add(left: &ExprNode, right: &ExprNode) -> String {
    match right {
        ExprNode::Num(n) if *n < 0.0 => {
            format!("{} - {}", render_expr(left), render_num(-n))
        }
        ExprNode::UnaryMinus(inner) => {
            format!("{} - {}", render_expr(left), render_atom(inner))
        }
        _ => format!("{} + {}", render_expr(left), render_expr(right)),
    }
}

/// Smart mul: `Num(c) * Var(v)` → `"cv"` (implicit), else explicit `*`.
fn render_mul(left: &ExprNode, right: &ExprNode) -> String {
    match (left, right) {
        (ExprNode::Num(c), ExprNode::Var(v)) => format!("{}{}", render_num(*c), v),
        (ExprNode::Num(c), ExprNode::Binary { op: BinaryOp::Pow, .. }) => {
            format!("{}{}", render_num(*c), render_atom(right))
        }
        _ => format!("{} * {}", render_atom(left), render_atom(right)),
    }
}
