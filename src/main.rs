mod core;

use std::env;
use std::fs;
use crate::core::{lexer::tokenizer::tokenize, parser::parse, interpreter::execute};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: mlang <file.mlang>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let source_code = fs::read_to_string(file_path)
        .unwrap_or_else(|_| {
            eprintln!("Error: Could not read file `{}`", file_path);
            std::process::exit(1);
        });

    let tokens = tokenize(&source_code).unwrap_or_else(|err| {
        eprintln!("Lexer Error: {}", err);
        std::process::exit(1);
    });

    let ast = parse(tokens).unwrap_or_else(|err| {
        eprintln!("Parser Error: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = execute(ast) {
        eprintln!("Runtime Error: {}", err);
        std::process::exit(1);
    }
}
