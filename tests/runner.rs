use mlang::core::{
    interpreter::execute,
    lexer::tokenizer::tokenize,
    parser::parse::entry::parse,
    resolver::resolve_imports,
};
use std::path::Path;

fn run_file(path: &str) -> String {
    let source = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("failed to read {}", path));
    let tokens = tokenize(&source)
        .unwrap_or_else(|e| panic!("tokenize failed for {}: {}", path, e));
    let ast = parse(tokens)
        .unwrap_or_else(|e| panic!("parse failed for {}: {}", path, e));
    let ast = resolve_imports(Path::new(path), ast)
        .unwrap_or_else(|e| panic!("resolve failed for {}: {}", path, e));
    let mut output = String::new();
    execute(ast, Some(&mut output))
        .unwrap_or_else(|e| panic!("execute failed for {}: {}", path, e));
    output
}

// ── interpreter tests ─────────────────────────────────────────────────────────

#[test]
fn test_arrays() {
    let out = run_file("tests/interpreter/arrays.mth");
    assert!(
        out.contains("All array tests passed"),
        "missing sentinel in arrays output:\n{}", out
    );
}

#[test]
fn test_functions() {
    let out = run_file("tests/interpreter/functions.mth");
    let lines: Vec<&str> = out.lines().collect();
    // greet() prints inside a function body → goes to stdout, not captured buffer
    assert_eq!(lines[0], "8",   "add(5,3) should be 8");
    assert_eq!(lines[1], "120", "factorial(5) should be 120");
    assert_eq!(lines[2], "8",   "fib(6) should be 8");
}

#[test]
fn test_loops() {
    let out = run_file("tests/interpreter/loops.mth");
    let lines: Vec<&str> = out.lines().collect();
    // while i<5
    assert_eq!(&lines[..5], &["0", "1", "2", "3", "4"]);
    // for j in 0..3
    assert_eq!(&lines[5..8], &["0", "1", "2"]);
    // break at x==3
    assert_eq!(&lines[8..11], &["0", "1", "2"]);
    // continue at k==2 → skips 2
    assert_eq!(&lines[11..15], &["0", "1", "3", "4"]);
    // for n in [10,20,30]
    assert_eq!(&lines[15..18], &["10", "20", "30"]);
}

#[test]
fn test_structs() {
    let out = run_file("tests/interpreter/structs.mth");
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "10", "p1.x");
    assert_eq!(lines[1], "20", "p1.y");
    assert_eq!(lines[2], "42", "p1.y after assign");
    assert_eq!(lines[3], "10", "rect.top_left.y");
    assert_eq!(lines[4], "10", "rect.bottom_right.x");
    assert_eq!(lines[5], "0",  "origin().x");
    assert_eq!(lines[6], "13", "add_points p3.x");
    assert_eq!(lines[7], "49", "add_points p3.y");
}

// ── math tests ────────────────────────────────────────────────────────────────

#[test]
fn test_math_linear() {
    let out = run_file("tests/math/linear.mth");
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "x = 3",  "2x + 4 = 10");
    assert_eq!(lines[1], "x = 4",  "3x - 12 = 0");
    assert_eq!(lines[2], "x = 5",  "x = 5");
    assert_eq!(lines[3], "x = 10", "x/2 = 5");
    assert_eq!(lines[4], "x = -8", "x + 8 = 0");
}

#[test]
fn test_math_quadratic() {
    let out = run_file("tests/math/quadratic.mth");
    let lines: Vec<&str> = out.lines().collect();
    // x^2 - 5x + 6 = 0 → roots 3 and 2 (order may vary)
    assert!(
        lines[0].contains("x = 3") && lines[0].contains("x = 2"),
        "x^2-5x+6=0 roots mismatch: {}", lines[0]
    );
    // x^2 - 4 = 0 → roots 2 and -2
    assert!(
        lines[1].contains("x = 2") && lines[1].contains("x = -2"),
        "x^2-4=0 roots mismatch: {}", lines[1]
    );
    // x^2 + 4x + 4 = 0 → double root -2
    assert_eq!(lines[2], "x = -2", "double root mismatch");
}

#[test]
fn test_math_symbolic() {
    let out = run_file("tests/math/symbolic.mth");
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "3x + 3",  "simplify 2x+x+4-1");
    assert_eq!(lines[1], "2x + 7",  "simplify 3x-x+5+2");
    assert!(
        lines[2].starts_with("28.27"),
        "evaluate pi*3^2 expected ~28.27, got {}", lines[2]
    );
    assert_eq!(lines[3], "14",    "evaluate 2+3*4");
    assert_eq!(lines[4], "4",     "evaluate sqrt(16)");
}

#[test]
fn test_modules() {
    let out = run_file("tests/modules/main.mth");
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "8",  "adder::add(5, 3)");
    assert_eq!(lines[1], "6",  "adder::subtract(10, 4)");
    assert_eq!(lines[2], "14", "math::double(7)");
    assert_eq!(lines[3], "16", "math::square(4)");
}
