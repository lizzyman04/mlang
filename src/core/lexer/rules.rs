use crate::core::parser::ast::{Expression, Type};
use std::collections::HashSet;

pub fn is_keyword(word: &str) -> bool {
    let keywords: HashSet<&str> = [
        "int", "dec", "txt", "bool", "void", "true", "false", "main", "print", "return",
        "if", "else", "array", "while", "for", "in", "break", "continue", "struct",
    ]
    .iter()
    .copied()
    .collect();

    keywords.contains(word)
}

pub fn is_variable_type(word: &str) -> bool {
    matches!(word, "int" | "dec" | "txt" | "bool" | "array" | "void")
}

pub fn infer_type(expr: &Expression) -> Type {
    match expr {
        Expression::IntLiteral(_) => Type::Int,
        Expression::DecLiteral(_) => Type::Dec,
        Expression::TxtLiteral(_) => Type::Txt,
        Expression::BoolLiteral(_) => Type::Bool,
        Expression::ArrayLiteral(elems) => {
            let inner = elems.first().map(infer_type).unwrap_or(Type::Void);
            Type::Array(Box::new(inner))
        }
        Expression::StructLiteral { name, .. } => Type::Struct(name.clone()),
        _ => Type::Void,
    }
}
