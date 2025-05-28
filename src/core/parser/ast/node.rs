use super::expr::Expression;

#[derive(Debug, Clone)]
pub enum ASTNode {
    FunctionDecl {
        name: String,
        body: Vec<ASTNode>,
    },
    PrintStmt {
        expression: Box<ASTNode>,
    },
    VarDecl {
        name: String,
        var_type: String,
        value: Box<ASTNode>,
    },
    Expression(Expression),
}
