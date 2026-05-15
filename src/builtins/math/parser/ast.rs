#[derive(Debug, Clone, PartialEq)]
pub enum ExprNode {
    Num(f64),
    Var(String),
    FuncCall { name: String, arg: Box<ExprNode> },
    Binary { op: BinaryOp, left: Box<ExprNode>, right: Box<ExprNode> },
    UnaryMinus(Box<ExprNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl BinaryOp {
    pub fn symbol(&self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Pow => "^",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Equation {
    pub lhs: Box<ExprNode>,
    pub rhs: Box<ExprNode>,
}
