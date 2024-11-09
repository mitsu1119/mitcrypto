use std::fmt::Display;

use rug::{integer::IsPrime, Integer};

use crate::error::MathError;

use super::finite_field_element::FiniteFieldElement;

type Result<T> = std::result::Result<T, MathError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteField {
    order: Integer,
}

impl FiniteField {
    pub fn new(order: Integer) -> Result<Self> {
        match order.is_probably_prime(100) {
            IsPrime::Yes | IsPrime::Probably => Ok(Self { order }),
            _ => Err(MathError::UnimplementedError(
                "finite field whose order is a composite number".to_string(),
            )),
        }
    }

    pub fn new_unchecked(order: Integer) -> Self {
        Self { order }
    }

    pub fn order(&self) -> &Integer {
        &self.order
    }

    pub fn elem(&self, x: Integer) -> FiniteFieldElement {
        FiniteFieldElement::new(self, x)
    }
}

impl Display for FiniteField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Finite Field of size {}", self.order())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::MathError;

    use super::FiniteField;
    use rug::Integer;

    #[test]
    fn finite_field() {
        let tests = [
            FiniteField::new(Integer::from(5)),
            FiniteField::new(Integer::from(10)),
            FiniteField::new(Integer::from(4)),
            FiniteField::new(Integer::from(100)),
        ];
        let res = [
            Ok(FiniteField {
                order: Integer::from(5),
            }),
            Err(MathError::UnimplementedError(
                "finite field whose order is a composite number".to_string(),
            )),
            Err(MathError::UnimplementedError(
                "finite field whose order is a composite number".to_string(),
            )),
            Err(MathError::UnimplementedError(
                "finite field whose order is a composite number".to_string(),
            )),
        ];

        for (t, r) in tests.iter().zip(&res) {
            assert_eq!(t, r);
        }
    }
}
