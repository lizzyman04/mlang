use crate::core::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    IntLiteral(i64),
    DecLiteral(f64),
    TxtLiteral(String),
    BoolLiteral(bool),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        operand: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    ArrayLiteral(Vec<Expression>),
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    MethodCall {
        object: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
}

pub enum ExecutionResult {
    Unit,
    Return(Expression),
}
