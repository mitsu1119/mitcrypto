use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathError {
    ValueError(String),
    TypeError(String),
    UnimplementedError(String),
}

impl Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueError(e) => write!(f, "Value Error: {}", e),
            Self::TypeError(e) => write!(f, "Type Error: {}", e),
            Self::UnimplementedError(e) => {
                write!(f, "Unimplemented Error: {} is not implemented", e)
            }
        }
    }
}
