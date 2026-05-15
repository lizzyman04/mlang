#[derive(Debug, Clone)]
pub struct TransformerError {
    pub message: String,
    pub step: usize,
}

impl TransformerError {
    pub fn new(message: &str, step: usize) -> Self {
        Self { message: message.to_string(), step }
    }
}

impl std::fmt::Display for TransformerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "transformer error at step {}: {}", self.step, self.message)
    }
}
