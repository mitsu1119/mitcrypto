use std::fmt::Display;

use rug::Integer;

use crate::error::MathError;

use super::mod_ring_element::ZmodElement;

type Result<T> = std::result::Result<T, MathError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zmod {
    order: Integer,
}

impl Zmod {
    pub fn new(order: Integer) -> Result<Self> {
        if order == 0 {
            Err(MathError::ValueError(
                "the order of Zmod must be not zero".into(),
            ))
        } else {
            Ok(Zmod { order: order.abs() })
        }
    }

    pub fn order(&self) -> &Integer {
        &self.order
    }

    pub fn elem(&self, x: Integer) -> ZmodElement {
        ZmodElement::new(self, x)
    }
}

impl Display for Zmod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ring of integers modulo {}", self.order())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::MathError;

    use rug::Integer;

    use super::Zmod;

    #[test]
    fn zmod() {
        let tests = [
            Zmod::new(Integer::from(5)),
            Zmod::new(Integer::from(-5)),
            Zmod::new(Integer::from(0)),
        ];
        let res = [
            Ok(Zmod {
                order: Integer::from(5),
            }),
            Ok(Zmod {
                order: Integer::from(5),
            }),
            Err(MathError::ValueError(
                "the order of Zmod must be not zero".into(),
            )),
        ];

        for (t, r) in tests.iter().zip(&res) {
            assert_eq!(t, r);
        }
    }
}
