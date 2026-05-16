use std::path::{Path, PathBuf};

use crate::core::lexer::tokenizer::tokenize;
use crate::core::parser::ast::ASTNode;
use crate::core::parser::parse::entry::parse_module_file;

pub struct Module {
    pub name: String,
    pub path: PathBuf,
    pub ast: Vec<ASTNode>,
}

pub fn parse_import(path: &str, current_dir: &Path) -> Result<Module, String> {
    let full_path = current_dir.join(path);
    let source = std::fs::read_to_string(&full_path)
        .map_err(|e| format!("Cannot read module '{}': {}", path, e))?;
    let tokens = tokenize(&source)
        .map_err(|e| format!("Lexer error in '{}': {}", path, e))?;
    let ast = parse_module_file(tokens)
        .map_err(|e| format!("Parse error in '{}': {}", path, e))?;

    let name = ast
        .iter()
        .find_map(|node| {
            if let ASTNode::ModuleDecl { name } = node {
                Some(name.clone())
            } else {
                None
            }
        })
        .ok_or_else(|| format!("Module file '{}' is missing a 'module <name>' declaration", path))?;

    Ok(Module { name, path: full_path, ast })
}
