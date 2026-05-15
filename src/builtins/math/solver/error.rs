use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
pub enum SolveError {
    NoVariable(String),
    MultipleVariables(HashSet<String>),
    NoSolution(String),
    InfiniteSolutions(String),
    Unsupported(String),
}

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolveError::NoVariable(msg) => write!(f, "{}", msg),
            SolveError::MultipleVariables(vars) => {
                let mut names: Vec<&str> = vars.iter().map(|s| s.as_str()).collect();
                names.sort();
                write!(f, "multiple variables found: {}", names.join(", "))
            }
            SolveError::NoSolution(msg) => write!(f, "no solution: {}", msg),
            SolveError::InfiniteSolutions(msg) => write!(f, "infinite solutions: {}", msg),
            SolveError::Unsupported(msg) => write!(f, "unsupported: {}", msg),
        }
    }
}
