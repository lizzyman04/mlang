pub struct MathExpr {
    original: String,
    variable: String,
    steps: Vec<String>,
    result: String,
}

impl MathExpr {
    pub fn new(original: String, variable: String, steps: Vec<String>, result: String) -> Self {
        Self { original, variable, steps, result }
    }

    pub fn result(&self) -> &str {
        &self.result
    }

    pub fn step(&self, n: usize) -> Option<&str> {
        self.steps.get(n).map(String::as_str)
    }

    pub fn steps(&self) -> &[String] {
        &self.steps
    }

    pub fn original(&self) -> &str {
        &self.original
    }

    pub fn variable(&self) -> &str {
        &self.variable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> MathExpr {
        MathExpr::new(
            "3x - 12 = 0".to_string(),
            "x".to_string(),
            vec!["Step 0: 3x - 12 = 0  # initial expression".to_string(),
                 "Step 1: 3x = 12  # add 12".to_string(),
                 "Step 2: x = 4  # divide by 3".to_string()],
            "x = 4".to_string(),
        )
    }

    #[test]
    fn test_original() {
        assert_eq!(make().original(), "3x - 12 = 0");
    }

    #[test]
    fn test_variable() {
        assert_eq!(make().variable(), "x");
    }

    #[test]
    fn test_result() {
        assert_eq!(make().result(), "x = 4");
    }

    #[test]
    fn test_steps_len() {
        assert_eq!(make().steps().len(), 3);
    }

    #[test]
    fn test_step_valid() {
        assert_eq!(make().step(1), Some("Step 1: 3x = 12  # add 12"));
    }

    #[test]
    fn test_step_out_of_bounds() {
        assert_eq!(make().step(99), None);
    }
}
