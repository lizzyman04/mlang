use std::collections::HashMap;
use crate::core::parser::ast::{Expression, Type};

pub struct Environment {
    variables: HashMap<String, (Type, Expression)>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, var_type: Type, value: Expression) {
        self.variables.insert(name, (var_type, value));
    }

    pub fn get(&self, name: &str) -> Option<&(Type, Expression)> {
        self.variables.get(name)
    }
}
