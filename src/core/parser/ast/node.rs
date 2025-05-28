use super::expr::Expression;

#[derive(Debug, Clone)]
pub enum ASTNode {
    FunctionDecl {
        name: String,
        body: Vec<ASTNode>,
        return_type: String,
        params: Vec<(String, String)>,
    },
    ReturnStmt {
        value: Box<ASTNode>,
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

