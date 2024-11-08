use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathError {
    ValueError(String),
}

impl Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueError(e) => write!(f, "Value Error: {}", e),
        }
    }
}
