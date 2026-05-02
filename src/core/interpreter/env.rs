use std::collections::HashMap;
use crate::core::parser::ast::{ASTNode, Expression, Type};

#[derive(Clone)]
pub struct FunctionDef {
    pub params: Vec<(Type, String)>,
    pub return_type: Type,
    pub body: Vec<ASTNode>,
}

pub struct Environment {
    variables: HashMap<String, (Type, Expression)>,
    pub functions: HashMap<String, FunctionDef>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, var_type: Type, value: Expression) {
        self.variables.insert(name, (var_type, value));
    }

    pub fn get(&self, name: &str) -> Option<&(Type, Expression)> {
        self.variables.get(name)
    }

    pub fn register_function(&mut self, name: String, def: FunctionDef) {
        self.functions.insert(name, def);
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(name)
    }

    pub fn child_for_call(&self) -> Self {
        Environment {
            variables: HashMap::new(),
            functions: self.functions.clone(),
        }
    }
}
