use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::core::parser::ast::ASTNode;

mod import;
use import::parse_import;

/// Resolves all `import` declarations in `program`, merging the imported
/// modules' function declarations (namespaced as `module::fn`) before the
/// main program. Returns the flattened AST ready for execution.
pub fn resolve_imports(main_path: &Path, program: Vec<ASTNode>) -> Result<Vec<ASTNode>, String> {
    let current_dir = main_path.parent().unwrap_or(Path::new("."));
    let mut result: Vec<ASTNode> = Vec::new();
    let mut visited: HashSet<PathBuf> = HashSet::new();

    let mut import_paths: Vec<String> = Vec::new();
    let mut rest: Vec<ASTNode> = Vec::new();

    for node in program {
        match node {
            ASTNode::ImportDecl { path } => import_paths.push(path),
            ASTNode::ModuleDecl { .. } => {}
            other => rest.push(other),
        }
    }

    for path_str in import_paths {
        let full_path = current_dir.join(&path_str);
        let canonical = full_path
            .canonicalize()
            .map_err(|_| format!("Cannot find module '{}'", path_str))?;

        if visited.contains(&canonical) {
            return Err(format!("Circular import detected: '{}'", path_str));
        }
        visited.insert(canonical);

        let module = parse_import(&path_str, current_dir)?;

        for node in module.ast {
            match node {
                ASTNode::FunctionDecl { name, body, return_type, params } => {
                    result.push(ASTNode::FunctionDecl {
                        name: format!("{}::{}", module.name, name),
                        body,
                        return_type,
                        params,
                    });
                }
                ASTNode::StructDecl { name, fields } => {
                    result.push(ASTNode::StructDecl { name, fields });
                }
                ASTNode::ModuleDecl { .. } => {}
                _ => {}
            }
        }
    }

    result.extend(rest);
    Ok(result)
}
