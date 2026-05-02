use super::expr::Expression;
use super::types::Type;

#[derive(Debug, Clone)]
pub enum ASTNode {
    FunctionDecl {
        name: String,
        body: Vec<ASTNode>,
        return_type: Type,
        params: Vec<(Type, String)>,
    },
    ReturnStmt {
        value: Box<ASTNode>,
    },
    PrintStmt {
        expression: Box<ASTNode>,
    },
    VarDecl {
        name: String,
        var_type: Type,
        value: Box<ASTNode>,
    },
    LetDecl {
        name: String,
        value: Box<ASTNode>,
    },
    IndexAssign {
        name: String,
        index: Box<ASTNode>,
        value: Box<ASTNode>,
    },
    ExprStmt(Box<ASTNode>),
    Expression(Expression),
    WhileLoop {
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    ForRangeLoop {
        var: String,
        start: Box<ASTNode>,
        end: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    ForArrayLoop {
        var: String,
        array: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    Break,
    Continue,
}
