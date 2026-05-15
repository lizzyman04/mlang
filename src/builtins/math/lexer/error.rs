#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub position: usize,
}

impl LexerError {
    pub fn new(message: &str, position: usize) -> Self {
        Self { message: message.to_string(), position }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lexer error at position {}: {}", self.position, self.message)
    }
}
