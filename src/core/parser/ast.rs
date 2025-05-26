#[derive(Debug)]
pub enum ASTNode {
    FunctionDecl {
        name: String,
        body: Vec<ASTNode>,
    },
    PrintStmt {
        expression: Box<ASTNode>,
    },
    IntLiteral(i64),
    TxtLiteral(String),
}

// #[derive(Debug, Clone)]
// pub enum Expression {
//     IntLiteral(i64),
//     StringLiteral(String),
// }
