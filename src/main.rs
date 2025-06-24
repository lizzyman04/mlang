mod core;
mod gui;

use crate::core::{interpreter::execute, lexer::tokenizer::tokenize, parser::parse::entry};
use std::{env, error::Error, fs};

fn format_error(kind: &str, file: &str, message: &str) -> String {
    format!(
        "\x1b[31m{}\x1b[0m in \x1b[33m`{}`\x1b[0m: {}",
        kind, file, message
    )
}

fn run_cli(file_path: &str) -> Result<(), Box<dyn Error>> {
    let source_code = fs::read_to_string(&file_path)
        .map_err(|_| format_error("Could not read file", &file_path, ""))?;

    let tokens = tokenize(&source_code)
        .map_err(|e| format_error("Lexer Error", &file_path, &e.to_string()))?;

    let ast = entry::parse(tokens)
        .map_err(|e| format_error("Parser Error", &file_path, &e.to_string()))?;

    execute(ast).map_err(|e| format_error("Runtime Error", &file_path, &e.to_string()))?;

    Ok(())
}

fn main() -> eframe::Result<()> {
    let mut args = env::args().skip(1);

    if let Some(first_arg) = args.next() {
        if first_arg == "--cli" {
            let file = args.next().unwrap_or_else(|| {
                eprintln!("\x1b[33mUsage: mlang --cli <file.mlang>\x1b[0m");
                std::process::exit(1);
            });

            println!("\n============ MLang — A Math-First Programming Language ============\n");

            let exit_code = match run_cli(&file) {
                Ok(_) => 0,
                Err(err) => {
                    eprintln!("{}", err);
                    1
                }
            };

            println!("\n===================================================================\n");
            std::process::exit(exit_code);
        }
    }

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "MLang IDE",
        options,
Box::new(|_cc| Ok::<_, Box<dyn std::error::Error + Send + Sync>>(Box::new(gui::AppState::default())))
    )
}
