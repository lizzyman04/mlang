use crate::core::lexer::token::TokenKind;

#[derive(Debug, Clone)]
pub enum Expression {
    IntLiteral(i64),
    DecLiteral(f64),
    TxtLiteral(String),
    BoolLiteral(bool),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: TokenKind,
        right: Box<Expression>,
    },
}

pub enum ExecutionResult {
    // Value(String),
    Unit,
    Return(Expression),
}
