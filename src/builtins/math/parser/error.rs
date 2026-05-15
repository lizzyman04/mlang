#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub position: usize,
}

impl ParserError {
    pub fn new(message: &str, position: usize) -> Self {
        Self { message: message.to_string(), position }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parser error at position {}: {}", self.position, self.message)
    }
}
