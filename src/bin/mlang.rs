use mlang::core::{
    interpreter::{env::Environment, execute, execute_repl_stmts},
    lexer::tokenizer::tokenize,
    parser::parse::entry::{parse, parse_stmts},
};
use std::{
    fs,
    io::{self, BufRead, Write},
};

fn color(code: &str, text: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", code, text)
}

fn run_file(path: &str) -> i32 {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("{}: could not read file '{}'", color("31", "Error"), path);
            return 1;
        }
    };

    let tokens = match tokenize(&source) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "{} in {}: {}",
                color("31", "Lexing Error"),
                color("33", &format!("`{}`", path)),
                e
            );
            return 1;
        }
    };

    let ast = match parse(tokens) {
        Ok(a) => a,
        Err(e) => {
            eprintln!(
                "{} in {}: {}",
                color("31", "Parsing Error"),
                color("33", &format!("`{}`", path)),
                e
            );
            return 1;
        }
    };

    match execute(ast, None) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!(
                "{} in {}: {}",
                color("31", "Runtime Error"),
                color("33", &format!("`{}`", path)),
                e
            );
            1
        }
    }
}

fn run_repl() {
    println!("{}", color("1", "MLang REPL — Math-First Language"));
    println!("Type {} to quit\n", color("33", "exit"));

    let stdin = io::stdin();
    let mut env = Environment::new();
    let mut buffer = String::new();
    let mut brace_depth: i32 = 0;

    loop {
        let prompt = if brace_depth > 0 { "..  " } else { ">>  " };
        print!("{}", color("32", prompt));
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }

        let trimmed = line.trim();

        if matches!(trimmed, "exit" | ":quit" | ":q") {
            break;
        }

        if trimmed.is_empty() && brace_depth == 0 {
            continue;
        }

        for ch in trimmed.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => brace_depth -= 1,
                _ => {}
            }
        }

        if !buffer.is_empty() {
            buffer.push('\n');
        }
        buffer.push_str(trimmed);

        if brace_depth > 0 {
            continue;
        }

        let input = std::mem::take(&mut buffer);
        brace_depth = 0;

        if input.trim().is_empty() {
            continue;
        }

        let tokens = match tokenize(&input) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}: {}", color("31", "Lexing Error"), e);
                continue;
            }
        };

        match parse_stmts(tokens) {
            Ok(stmts) => {
                if let Err(e) = execute_repl_stmts(stmts, &mut env) {
                    eprintln!("{}: {}", color("31", "Runtime Error"), e);
                }
            }
            Err(e) => {
                eprintln!("{}: {}", color("31", "Parsing Error"), e);
            }
        }
    }

    println!("\nGoodbye!");
}

fn main() {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("repl") => run_repl(),
        Some(path) => std::process::exit(run_file(path)),
        None => {
            eprintln!("Usage:");
            eprintln!("  mlang <file.mth>   Run a .mth source file");
            eprintln!("  mlang repl         Start interactive REPL");
            std::process::exit(1);
        }
    }
}
