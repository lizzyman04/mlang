use std::collections::HashSet;

use super::divide::divide_coefficient;
use super::step::{record_step, render_expr, TransformStep};
use crate::builtins::math::parser::{BinaryOp, ExprNode};

// ── variable collection ───────────────────────────────────────────────────────

pub fn collect_vars(node: &ExprNode) -> HashSet<String> {
    match node {
        ExprNode::Var(name) => {
            let mut s = HashSet::new();
            s.insert(name.clone());
            s
        }
        ExprNode::Binary { left, right, .. } => {
            let mut s = collect_vars(left);
            s.extend(collect_vars(right));
            s
        }
        ExprNode::UnaryMinus(inner) => collect_vars(inner),
        ExprNode::FuncCall { arg, .. } => collect_vars(arg),
        _ => HashSet::new(),
    }
}

// ── term flattening ───────────────────────────────────────────────────────────

/// A flat term: (outer_coefficient, Option<base_ExprNode>).
/// base=None  → constant term.
/// base=Some  → symbolic term (may itself contain a div or pow).
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

// ── isolation ─────────────────────────────────────────────────────────────────

/// Isolates `target_var` in a normalized expression (`= 0`).
///
/// Returns the numeric solution and every transformation step.
///
/// Algorithm (linear only):
///   1. Flatten into (coeff, base) terms
///   2. Separate var terms from constant terms
///   3. Sum constant coefficients  → move to RHS
///   4. Sum effective var coefficients via divide_coefficient
///   5. Divide RHS by effective coefficient
pub fn isolate_variable(
    node: &ExprNode,
    target_var: &str,
) -> Result<(ExprNode, Vec<TransformStep>), String> {
    let mut steps: Vec<TransformStep> = Vec::new();

    let terms = flatten(node, 1.0);

    let mut var_terms: Vec<(f64, ExprNode)> = Vec::new();
    let mut const_sum: f64 = 0.0;

    for (coeff, base) in terms {
        match base {
            None => const_sum += coeff,
            Some(base_expr) => var_terms.push((coeff, base_expr)),
        }
    }

    if var_terms.is_empty() {
        return Err("no variable terms found in expression".to_string());
    }

    // ── step 0: initial expression = 0 ───────────────────────────────────────
    record_step(
        &mut steps,
        format!("{} = 0", render_expr(node)),
        "initial expression",
    );

    // ── compute effective coefficient ─────────────────────────────────────────
    let mut effective_coeff: f64 = 0.0;
    for (outer, base) in &var_terms {
        let inner = divide_coefficient(base, target_var)
            .map_err(|e| format!("unsupported term structure: {}", e))?;
        effective_coeff += outer * inner;
    }

    if effective_coeff.abs() < 1e-12 {
        return Err(format!(
            "coefficient of '{}' is zero — equation has no unique solution",
            target_var
        ));
    }

    // ── step 1: move constants to RHS ─────────────────────────────────────────
    let rhs_after_move = -const_sum;

    // Rebuild just the var side for display
    let var_lhs_str = if var_terms.len() == 1 {
        let (outer, ref base) = var_terms[0];
        if (outer - 1.0).abs() < 1e-12 {
            render_expr(base)
        } else {
            format!("{}{}", outer as i64, render_expr(base))
        }
    } else {
        // Multiple var terms — rare after combine, render them manually
        var_terms
            .iter()
            .enumerate()
            .map(|(i, (c, b))| {
                if i == 0 {
                    if (*c - 1.0).abs() < 1e-12 {
                        render_expr(b)
                    } else {
                        format!("{}{}", *c as i64, render_expr(b))
                    }
                } else if *c < 0.0 {
                    format!(" - {}{}", (-c) as i64, render_expr(b))
                } else {
                    format!(" + {}{}", *c as i64, render_expr(b))
                }
            })
            .collect::<String>()
    };

    let rhs_str = render_num(rhs_after_move);

    if const_sum.abs() > 1e-12 {
        let rule = if const_sum < 0.0 {
            format!("add {} to both sides", render_num(-const_sum))
        } else {
            format!("subtract {} from both sides", render_num(const_sum))
        };
        record_step(
            &mut steps,
            format!("{} = {}", var_lhs_str, rhs_str),
            &rule,
        );
    }

    // ── step 2: divide by coefficient ────────────────────────────────────────
    let solution_val = rhs_after_move / effective_coeff;
    let solution_node = ExprNode::Num(solution_val);

    if (effective_coeff - 1.0).abs() > 1e-12 {
        let rule = format!("divide both sides by {}", render_num(effective_coeff));
        record_step(
            &mut steps,
            format!("{} = {}", target_var, render_num(solution_val)),
            &rule,
        );
    } else if const_sum.abs() < 1e-12 {
        // Coefficient is 1 and no constants — only had step 0, add a trivial final step
        record_step(
            &mut steps,
            format!("{} = {}", target_var, render_num(solution_val)),
            "solution",
        );
    }

    Ok((solution_node, steps))
}

fn render_num(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}
