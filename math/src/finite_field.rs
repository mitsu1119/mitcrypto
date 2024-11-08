use rug::{integer::IsPrime, Integer};

use crate::error::MathError;

type Result<T> = std::result::Result<T, MathError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteField {
    p: Integer,
}

impl FiniteField {
    pub fn new(p: Integer) -> Result<Self> {
        match p.is_probably_prime(100) {
            IsPrime::Yes | IsPrime::Probably => Ok(Self { p }),
            _ => Err(MathError::ValueError(
                "the order of a finite field mut be a prime".to_string(),
            )),
        }
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
        ];
        let res = [
            Ok(FiniteField {
                p: Integer::from(5),
            }),
            Err(MathError::ValueError(
                "the order of a finite field mut be a prime".to_string(),
            )),
        ];

        for (t, r) in tests.iter().zip(&res) {
            assert_eq!(t, r);
        }
    }
}
