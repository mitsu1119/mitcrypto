use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathError {
    ValueError(String),
    TypeError(String),
    ZeroDivisionError(String),
    UnimplementedError(String),
}

impl MathError {
    pub fn unsupported_operand<T: Display, U: Display>(op: &str, x: T, y: U) -> MathError {
        Self::TypeError(format!(
            "unsupported operand parent(s) for {}: {} and {}",
            op, x, y
        ))
    }
}

impl Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueError(e) => write!(f, "Value Error: {}", e),
            Self::TypeError(e) => write!(f, "Type Error: {}", e),
            MathError::ZeroDivisionError(e) => write!(f, "ZeroDivision Error: {}", e),
            Self::UnimplementedError(e) => {
                write!(f, "Unimplemented Error: {} is not implemented", e)
            }
        }
    }
}
