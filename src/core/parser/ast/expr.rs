#[derive(Debug, Clone)]
pub enum Expression {
    IntLiteral(i64),
    DecLiteral(f64),
    TxtLiteral(String),
    Identifier(String),
}
