mod token;
mod number;
mod identifier;
mod operator;
mod error;

pub use token::MathToken;
pub use error::LexerError;

use number::parse_number;
use identifier::parse_identifier;
use operator::parse_operator;

/// Tokenizes a math equation string into a flat list of MathTokens.
///
/// Implicit multiplication is inserted between adjacent tokens where applicable:
/// - `2x`  → `Num(2), Ident("x")` (caller handles implicit multiply at parse time)
/// - `sqrt(x)` → `Func("sqrt"), LParen, Ident("x"), RParen`
///
/// Constants are substituted during lexing:
/// - `pi` → `Num(3.141592653589793)`
/// - `e`  → `Num(2.718281828459045)`
/// - `tau` → `Num(6.283185307179586)`
/// - `inf` → `Num(∞)`
pub fn tokenize_equation(input: &str) -> Result<Vec<MathToken>, String> {
    let mut tokens: Vec<MathToken> = Vec::new();
    let mut chars = input.chars().peekable();
    let mut pos: usize = 0;

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            pos += 1;
            continue;
        }

        // Number: digit or leading dot
        if ch.is_ascii_digit() || ch == '.' {
            let tok = parse_number(&mut chars).map_err(|e| format!("at position {}: {}", pos, e))?;

            // Implicit multiplication: Num followed immediately by Ident/LParen/Func
            // e.g. "2x" → Num(2), Ident("x")  — insert Num, then Ident is lexed next turn.
            // We do NOT insert a Star token here; that is the parser's job.
            tokens.push(tok);
            continue;
        }

        // Identifier or keyword (variable, constant, function name)
        if ch.is_alphabetic() || ch == '_' {
            let tok = parse_identifier(&mut chars).map_err(|e| format!("at position {}: {}", pos, e))?;
            tokens.push(tok);
            continue;
        }

        // Operator or punctuation
        chars.next();
        match parse_operator(ch, &mut chars) {
            Some(tok) => tokens.push(tok),
            None => {
                return Err(LexerError::new(
                    &format!("unexpected character '{}'", ch),
                    pos,
                )
                .to_string());
            }
        }

        pos += 1;
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use MathToken::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn num_approx(tok: &MathToken, val: f64) -> bool {
        matches!(tok, Num(n) if approx_eq(*n, val))
    }

    #[test]
    fn test_linear_equation() {
        // "2x + 4 = 10" → [Num(2), Ident("x"), Plus, Num(4), Eq, Num(10)]
        let tokens = tokenize_equation("2x + 4 = 10").unwrap();
        assert!(num_approx(&tokens[0], 2.0));
        assert_eq!(tokens[1], Ident("x".to_string()));
        assert_eq!(tokens[2], Plus);
        assert!(num_approx(&tokens[3], 4.0));
        assert_eq!(tokens[4], Eq);
        assert!(num_approx(&tokens[5], 10.0));
        assert_eq!(tokens.len(), 6);
    }

    #[test]
    fn test_function_call() {
        // "sqrt(x) = 4" → [Func("sqrt"), LParen, Ident("x"), RParen, Eq, Num(4)]
        let tokens = tokenize_equation("sqrt(x) = 4").unwrap();
        assert_eq!(tokens[0], Func("sqrt".to_string()));
        assert_eq!(tokens[1], LParen);
        assert_eq!(tokens[2], Ident("x".to_string()));
        assert_eq!(tokens[3], RParen);
        assert_eq!(tokens[4], Eq);
        assert!(num_approx(&tokens[5], 4.0));
        assert_eq!(tokens.len(), 6);
    }

    #[test]
    fn test_pi_constant_substitution() {
        // "pi * r^2 = 100" → [Num(pi), Star, Ident("r"), Caret, Num(2), Eq, Num(100)]
        let tokens = tokenize_equation("pi * r^2 = 100").unwrap();
        assert!(num_approx(&tokens[0], std::f64::consts::PI));
        assert_eq!(tokens[1], Star);
        assert_eq!(tokens[2], Ident("r".to_string()));
        assert_eq!(tokens[3], Caret);
        assert!(num_approx(&tokens[4], 2.0));
        assert_eq!(tokens[5], Eq);
        assert!(num_approx(&tokens[6], 100.0));
        assert_eq!(tokens.len(), 7);
    }

    #[test]
    fn test_polynomial() {
        // "x^2 - 4x + 4 = 0"
        let tokens = tokenize_equation("x^2 - 4x + 4 = 0").unwrap();
        assert_eq!(tokens[0], Ident("x".to_string()));
        assert_eq!(tokens[1], Caret);
        assert!(num_approx(&tokens[2], 2.0));
        assert_eq!(tokens[3], Minus);
        assert!(num_approx(&tokens[4], 4.0));
        assert_eq!(tokens[5], Ident("x".to_string()));
        assert_eq!(tokens[6], Plus);
        assert!(num_approx(&tokens[7], 4.0));
        assert_eq!(tokens[8], Eq);
        assert!(num_approx(&tokens[9], 0.0));
        assert_eq!(tokens.len(), 10);
    }

    #[test]
    fn test_decimal_number() {
        let tokens = tokenize_equation("3.14 = x").unwrap();
        assert!(num_approx(&tokens[0], 3.14));
    }

    #[test]
    fn test_tau_constant() {
        let tokens = tokenize_equation("tau").unwrap();
        assert!(num_approx(&tokens[0], std::f64::consts::TAU));
    }

    #[test]
    fn test_e_constant() {
        let tokens = tokenize_equation("e^x = 1").unwrap();
        assert!(num_approx(&tokens[0], std::f64::consts::E));
        assert_eq!(tokens[1], Caret);
        assert_eq!(tokens[2], Ident("x".to_string()));
    }

    #[test]
    fn test_all_functions_recognized() {
        for func in &["sqrt", "sin", "cos", "tan", "log", "exp", "abs", "pow"] {
            let input = format!("{}(x)", func);
            let tokens = tokenize_equation(&input).unwrap();
            assert_eq!(tokens[0], Func(func.to_string()), "failed for {}", func);
        }
    }

    #[test]
    fn test_unknown_char_errors() {
        assert!(tokenize_equation("x @ 2").is_err());
        assert!(tokenize_equation("x # y").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(MathToken::Plus.display(), "+");
        assert_eq!(MathToken::Num(3.0).display(), "3");
        assert_eq!(MathToken::Ident("x".to_string()).display(), "x");
        assert_eq!(MathToken::Func("sqrt".to_string()).display(), "sqrt");
    }
}
