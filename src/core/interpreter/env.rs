use std::collections::HashMap;
use crate::core::parser::ast::Expression;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, (String, Expression)>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, var_type: String, value: Expression) {
        self.variables.insert(name, (var_type, value));
    }

    pub fn get(&self, name: &str) -> Option<&(String, Expression)> {
        self.variables.get(name)
    }
}
