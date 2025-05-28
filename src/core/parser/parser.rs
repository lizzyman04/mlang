use crate::core::lexer::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    pub fn consume(&mut self, expected: &TokenKind) -> Result<(), String> {
        if let Some(token) = self.advance() {
            if &token.kind == expected {
                Ok(())
            } else {
                Err(format!(
                    "Expected token {:?} but found {:?} at line {}, column {}",
                    expected, token.kind, token.line, token.column
                ))
            }
        } else {
            Err(format!("Expected token {:?}, but found end of input", expected))
        }
    }

    pub fn consume_keyword(&mut self, expected: &str) -> Result<(), String> {
        if let Some(token) = self.advance() {
            match &token.kind {
                TokenKind::Keyword(kw) if kw == expected => Ok(()),
                _ => Err(format!(
                    "Expected keyword '{}' but found {:?} at line {}, column {}",
                    expected, token.kind, token.line, token.column
                )),
            }
        } else {
            Err(format!("Expected keyword '{}', but found end of input", expected))
        }
    }
}
