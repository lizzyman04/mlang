use crate::core::{
    interpreter::executor::execute, lexer::tokenizer::tokenize, parser::parse::entry,
};
use crate::gui::tabs::tabbed::Tab;

pub fn run_code(code: &str, output: &mut String, error: &mut String) -> Tab {
    *output = String::new();
    *error = String::new();

    let tokens = match tokenize(code) {
        Ok(tokens) => tokens,
        Err(e) => {
            *error = format!("Lexer Error: {}", e);
            return Tab::Errors;
        }
    };

    let ast = match entry::parse(tokens) {
        Ok(ast) => ast,
        Err(e) => {
            *error = format!("Parser Error: {}", e);
            return Tab::Errors;
        }
    };

    match execute(ast, Some(output)) {
        Ok(()) => Tab::Output,
        Err(e) => {
            *error = format!("Runtime Error: {}", e);
            Tab::Errors
        }
    }
}
