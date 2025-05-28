mod core;

use crate::core::{interpreter::execute, lexer::tokenizer::tokenize, parser::parse::entry};
use std::{env, error::Error, fs};

fn format_error(kind: &str, file: &str, message: &str) -> String {
    format!(
        "\x1b[31m{}\x1b[0m in \x1b[33m`{}`\x1b[0m: {}",
        kind, file, message
    )
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = env::args()
        .nth(1)
        .ok_or("\x1b[33mUsage: mlang <file.mlang>\x1b[0m")?;

    let source_code = fs::read_to_string(&file_path)
        .map_err(|_| format_error("Could not read file", &file_path, ""))?;

    let tokens = tokenize(&source_code)
        .map_err(|e| format_error("Lexer Error", &file_path, &e.to_string()))?;

    let ast = entry::parse(tokens)
        .map_err(|e| format_error("Parser Error", &file_path, &e.to_string()))?;

    execute(ast).map_err(|e| format_error("Runtime Error", &file_path, &e.to_string()))?;

    Ok(())
}

fn main() {
    println!("\n============ MLang — A Math-First Programming Language ============\n");

    let exit_code = match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    };

    println!("\n===================================================================\n");
    std::process::exit(exit_code);
}
