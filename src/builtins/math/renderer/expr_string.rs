use crate::builtins::math::{parser::ExprNode, transformer::render_expr};

pub fn expr_to_string(node: &ExprNode) -> String {
    render_expr(node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::math::parser::{BinaryOp, ExprNode};

    #[test]
    fn test_num() {
        assert_eq!(expr_to_string(&ExprNode::Num(3.14)), "3.14");
    }

    #[test]
    fn test_var() {
        assert_eq!(expr_to_string(&ExprNode::Var("x".to_string())), "x");
    }

    #[test]
    fn test_implicit_mul() {
        let node = ExprNode::Binary {
            op: BinaryOp::Mul,
            left: Box::new(ExprNode::Num(2.0)),
            right: Box::new(ExprNode::Var("x".to_string())),
        };
        assert_eq!(expr_to_string(&node), "2x");
    }

    #[test]
    fn test_pow() {
        let node = ExprNode::Binary {
            op: BinaryOp::Pow,
            left: Box::new(ExprNode::Var("x".to_string())),
            right: Box::new(ExprNode::Num(2.0)),
        };
        assert_eq!(expr_to_string(&node), "x^2");
    }

    #[test]
    fn test_func_call() {
        let node = ExprNode::FuncCall {
            name: "sqrt".to_string(),
            arg: Box::new(ExprNode::Var("x".to_string())),
        };
        assert_eq!(expr_to_string(&node), "sqrt(x)");
    }

    #[test]
    fn test_unary_minus() {
        let node = ExprNode::UnaryMinus(Box::new(ExprNode::Var("a".to_string())));
        assert_eq!(expr_to_string(&node), "-a");
    }
}
