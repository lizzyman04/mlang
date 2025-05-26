use std::fmt;
use crate::core::lexer::token::Token;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError at line {}, column {}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub fn new(token: &Token, message: String) -> Self {
        Self {
            message,
            line: token.line,
            column: token.column,
        }
    }

    pub fn eof(message: &str) -> Self {
        Self {
            message: message.to_string(),
            line: usize::MAX,
            column: usize::MAX,
        }
    }

    pub fn unexpected_token(token: &Token, expected: &str) -> Self {
        Self {
            message: format!("Expected {}, but found {:?}", expected, token.kind),
            line: token.line,
            column: token.column,
        }
    }
}
