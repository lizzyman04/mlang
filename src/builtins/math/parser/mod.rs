mod ast;
mod precedence;
mod expr;
mod equation;
mod error;

pub use ast::{BinaryOp, Equation, ExprNode};
pub use error::ParserError;

use equation::parse_equation;

pub fn parse_equation_from_tokens(
    tokens: &[crate::builtins::math::lexer::MathToken],
) -> Result<Equation, String> {
    parse_equation(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::math::lexer::tokenize_equation;

    fn parse(input: &str) -> Equation {
        let tokens = tokenize_equation(input).unwrap();
        parse_equation_from_tokens(&tokens).unwrap()
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    // ── helpers to destructure nodes ──────────────────────────────────────────

    fn as_num(node: &ExprNode) -> f64 {
        match node {
            ExprNode::Num(n) => *n,
            other => panic!("expected Num, got {:?}", other),
        }
    }

    fn as_var<'a>(node: &'a ExprNode) -> &'a str {
        match node {
            ExprNode::Var(s) => s,
            other => panic!("expected Var, got {:?}", other),
        }
    }

    fn as_binary(node: &ExprNode) -> (&BinaryOp, &ExprNode, &ExprNode) {
        match node {
            ExprNode::Binary { op, left, right } => (op, left.as_ref(), right.as_ref()),
            other => panic!("expected Binary, got {:?}", other),
        }
    }

    fn as_func(node: &ExprNode) -> (&str, &ExprNode) {
        match node {
            ExprNode::FuncCall { name, arg } => (name.as_str(), arg.as_ref()),
            other => panic!("expected FuncCall, got {:?}", other),
        }
    }

    // ── tests ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_linear_equation() {
        // "2x + 4 = 10"
        // LHS: Binary(Add, Binary(Mul, Num(2), Var("x")), Num(4))
        // RHS: Num(10)
        let eq = parse("2x + 4 = 10");

        let (add_op, add_left, add_right) = as_binary(&eq.lhs);
        assert_eq!(add_op, &BinaryOp::Add);
        assert!(approx(as_num(add_right), 4.0));

        let (mul_op, mul_left, mul_right) = as_binary(add_left);
        assert_eq!(mul_op, &BinaryOp::Mul);
        assert!(approx(as_num(mul_left), 2.0));
        assert_eq!(as_var(mul_right), "x");

        // RHS
        assert!(approx(as_num(&eq.rhs), 10.0));
    }

    #[test]
    fn test_function_call() {
        // "sqrt(x) = 4"
        let eq = parse("sqrt(x) = 4");
        let (name, arg) = as_func(&eq.lhs);
        assert_eq!(name, "sqrt");
        assert_eq!(as_var(arg), "x");
        assert!(approx(as_num(&eq.rhs), 4.0));
    }

    #[test]
    fn test_polynomial_precedence() {
        // "x^2 - 4x + 4 = 0"
        // LHS: Binary(Add, Binary(Sub, Binary(Pow, Var(x), Num(2)), Binary(Mul, Num(4), Var(x))), Num(4))
        let eq = parse("x^2 - 4x + 4 = 0");

        let (add_op, add_left, add_right) = as_binary(&eq.lhs);
        assert_eq!(add_op, &BinaryOp::Add);
        assert!(approx(as_num(add_right), 4.0));

        let (sub_op, sub_left, sub_right) = as_binary(add_left);
        assert_eq!(sub_op, &BinaryOp::Sub);

        // sub_left: x^2
        let (pow_op, pow_left, pow_right) = as_binary(sub_left);
        assert_eq!(pow_op, &BinaryOp::Pow);
        assert_eq!(as_var(pow_left), "x");
        assert!(approx(as_num(pow_right), 2.0));

        // sub_right: 4x (implicit mul)
        let (mul_op, mul_left, mul_right) = as_binary(sub_right);
        assert_eq!(mul_op, &BinaryOp::Mul);
        assert!(approx(as_num(mul_left), 4.0));
        assert_eq!(as_var(mul_right), "x");

        // RHS: 0
        assert!(approx(as_num(&eq.rhs), 0.0));
    }

    #[test]
    fn test_pi_star_r_pow_2() {
        // "pi * r^2 = 100"
        // LHS: Binary(Mul, Num(pi), Binary(Pow, Var(r), Num(2)))
        let eq = parse("pi * r^2 = 100");

        let (mul_op, mul_left, mul_right) = as_binary(&eq.lhs);
        assert_eq!(mul_op, &BinaryOp::Mul);
        assert!(approx(as_num(mul_left), std::f64::consts::PI));

        let (pow_op, pow_left, pow_right) = as_binary(mul_right);
        assert_eq!(pow_op, &BinaryOp::Pow);
        assert_eq!(as_var(pow_left), "r");
        assert!(approx(as_num(pow_right), 2.0));

        assert!(approx(as_num(&eq.rhs), 100.0));
    }

    #[test]
    fn test_unary_minus() {
        // "-x = 5"
        let eq = parse("-x = 5");
        match eq.lhs.as_ref() {
            ExprNode::UnaryMinus(inner) => assert_eq!(as_var(inner), "x"),
            other => panic!("expected UnaryMinus, got {:?}", other),
        }
    }

    #[test]
    fn test_parentheses() {
        // "(x + 1) * 2 = 4"
        let eq = parse("(x + 1) * 2 = 4");
        let (op, left, right) = as_binary(&eq.lhs);
        assert_eq!(op, &BinaryOp::Mul);
        let (inner_op, il, ir) = as_binary(left);
        assert_eq!(inner_op, &BinaryOp::Add);
        assert_eq!(as_var(il), "x");
        assert!(approx(as_num(ir), 1.0));
        assert!(approx(as_num(right), 2.0));
    }

    #[test]
    fn test_right_associative_pow() {
        // "x^2^3 = 0" → Binary(Pow, Var(x), Binary(Pow, Num(2), Num(3)))
        let eq = parse("x^2^3 = 0");
        let (op, left, right) = as_binary(&eq.lhs);
        assert_eq!(op, &BinaryOp::Pow);
        assert_eq!(as_var(left), "x");
        let (inner_op, il, ir) = as_binary(right);
        assert_eq!(inner_op, &BinaryOp::Pow);
        assert!(approx(as_num(il), 2.0));
        assert!(approx(as_num(ir), 3.0));
    }

    #[test]
    fn test_no_eq_errors() {
        let tokens = crate::builtins::math::lexer::tokenize_equation("x + 2").unwrap();
        assert!(parse_equation_from_tokens(&tokens).is_err());
    }

    #[test]
    fn test_implicit_mul_with_paren() {
        // "2(x + 1) = 0"
        let eq = parse("2(x + 1) = 0");
        let (op, left, right) = as_binary(&eq.lhs);
        assert_eq!(op, &BinaryOp::Mul);
        assert!(approx(as_num(left), 2.0));
        // right: (x + 1)
        let (inner_op, il, ir) = as_binary(right);
        assert_eq!(inner_op, &BinaryOp::Add);
        assert_eq!(as_var(il), "x");
        assert!(approx(as_num(ir), 1.0));
    }

    #[test]
    fn test_nested_function() {
        // "sqrt(x^2) = 3"
        let eq = parse("sqrt(x^2) = 3");
        let (name, arg) = as_func(&eq.lhs);
        assert_eq!(name, "sqrt");
        let (op, base, exp) = as_binary(arg);
        assert_eq!(op, &BinaryOp::Pow);
        assert_eq!(as_var(base), "x");
        assert!(approx(as_num(exp), 2.0));
    }
}
