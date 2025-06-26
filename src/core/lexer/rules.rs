use crate::core::parser::ast::Expression;
use std::collections::HashSet;

pub fn is_keyword(word: &str) -> bool {
    let keywords: HashSet<&str> = [
        "int", "dec", "txt", "bool", "true", "false", "main", "print", "return", "if", "else",
    ]
    .iter()
    .copied()
    .collect();

    keywords.contains(word)
}

pub fn is_variable_type(word: &str) -> bool {
    matches!(word, "int" | "dec" | "txt" | "bool")
}

pub fn infer_type_expr(expr: &Expression) -> &str {
    match expr {
        Expression::IntLiteral(_) => "int",
        Expression::DecLiteral(_) => "dec",
        Expression::TxtLiteral(_) => "txt",
        Expression::BoolLiteral(_) => "bool",
        _ => "unknown",
    }
}
