use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CipherError {
    ValueError(String),
}

impl Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueError(e) => write!(f, "Value Error: {}", e),
        }
    }
}
