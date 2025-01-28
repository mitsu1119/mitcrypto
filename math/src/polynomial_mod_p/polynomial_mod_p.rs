use std::fmt::Display;

use crate::{error::MathError, finite_field::finite_field::FiniteField};

type Result<T> = std::result::Result<T, MathError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolynomialModp {
    base: FiniteField,
}

impl PolynomialModp {
    pub fn new(base: FiniteField) -> Self {
        Self { base }
    }

    pub fn base(&self) -> &FiniteField {
        &self.base
    }
}

impl Display for PolynomialModp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Univariate polynomial ring over {}", self.base)
    }
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::finite_field::finite_field::FiniteField;

    use super::PolynomialModp;

    #[test]
    fn polynomial_mod_p() {
        let f = FiniteField::new(Integer::from(13)).unwrap();
        let fx = PolynomialModp::new(f);

        println!("{}", fx);

        panic!();
    }
}
