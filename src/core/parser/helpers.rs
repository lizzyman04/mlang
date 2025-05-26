use super::{parser::Parser, error::ParseError};
use crate::core::lexer::token::TokenKind;

impl Parser {
    pub fn expect(&mut self, expected: TokenKind) -> Result<(), ParseError> {
        if let Some(token) = self.advance() {
            if token.kind == expected {
                Ok(())
            } else {
                Err(ParseError::new(
                    token,
                    format!("Expected token '{:?}', found '{:?}'", expected, token.kind),
                ))
            }
        } else {
            Err(ParseError::eof(&format!("Expected token '{:?}' but found EOF", expected)))
        }
    }

    pub fn expect_keyword(&mut self, keyword: &str) -> Result<(), ParseError> {
        if let Some(token) = self.advance() {
            match &token.kind {
                TokenKind::Keyword(k) if k == keyword => Ok(()),
                _ => Err(ParseError::new(token, format!("Expected keyword '{}'", keyword))),
            }
        } else {
            Err(ParseError::eof(&format!("Expected keyword '{}' but found EOF", keyword)))
        }
    }

    pub fn expect_identifier(&mut self, ident: &str) -> Result<(), ParseError> {
        if let Some(token) = self.advance() {
            match &token.kind {
                TokenKind::Identifier(name) if name == ident => Ok(()),
                _ => Err(ParseError::new(token, format!("Expected identifier '{}'", ident))),
            }
        } else {
            Err(ParseError::eof(&format!("Expected identifier '{}' but found EOF", ident)))
        }
    }

    pub fn expect_token(&mut self, expected: &str) -> Result<(), ParseError> {
        if let Some(token) = self.advance() {
            let matches = match &token.kind {
                TokenKind::Symbol(sym) => {
                    expected.len() == 1 && *sym == expected.chars().next().unwrap()
                }
                TokenKind::Keyword(kw) => kw == expected,
                _ => false,
            };

            if matches {
                Ok(())
            } else {
                Err(ParseError::new(
                    token,
                    format!("Expected token '{}', found {:?}", expected, token.kind),
                ))
            }
        } else {
            Err(ParseError::eof(&format!("Expected token '{}', but got EOF", expected)))
        }
    }

    pub fn check_token(&self, expected: &str) -> bool {
        if let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Symbol(sym) => {
                    expected.len() == 1 && *sym == expected.chars().next().unwrap()
                }
                TokenKind::Keyword(kw) => kw == expected,
                _ => false,
            }
        } else {
            false
        }
    }
}
