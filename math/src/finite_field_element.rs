use core::panic;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use rug::Integer;

use crate::{error::MathError, finite_field::FiniteField};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteFieldElement<'a> {
    parent: &'a FiniteField,
    x: Integer,
}

impl<'a> FiniteFieldElement<'a> {
    pub fn new(parent: &'a FiniteField, x: Integer) -> Self {
        if x >= 0 {
            Self {
                parent,
                x: x % parent.order(),
            }
        } else {
            Self {
                parent,
                x: parent.order() - -x % parent.order(),
            }
        }
    }

    fn add(&mut self, rhs: Self) {
        if self.parent != rhs.parent {
            panic!(
                "{}",
                MathError::unsupported_operand("+", self.parent, rhs.parent)
            );
        }

        self.x += rhs.x;
        if &self.x >= self.parent.order() {
            self.x -= self.parent.order()
        }
    }

    fn sub(&mut self, rhs: Self) {
        if self.parent != rhs.parent {
            panic!(
                "{}",
                MathError::unsupported_operand("-", self.parent, rhs.parent)
            )
        }

        self.x -= rhs.x;
        if self.x < 0 {
            self.x += self.parent.order();
        }
    }

    fn mul(&mut self, rhs: Self) {
        if self.parent != rhs.parent {
            panic!(
                "{}",
                MathError::unsupported_operand("*", self.parent, rhs.parent)
            )
        }

        self.x *= rhs.x;
        if &self.x >= self.parent.order() {
            self.x %= self.parent.order();
        }
    }
}

impl<'a> AddAssign for FiniteFieldElement<'a> {
    fn add_assign(&mut self, rhs: Self) {
        self.add(rhs);
    }
}

impl<'a> Add for FiniteFieldElement<'a> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<'a> SubAssign for FiniteFieldElement<'a> {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub(rhs);
    }
}

impl<'a> Sub for FiniteFieldElement<'a> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl<'a> MulAssign for FiniteFieldElement<'a> {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul(rhs);
    }
}

impl<'a> Mul for FiniteFieldElement<'a> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl<'a> Display for FiniteFieldElement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}

#[cfg(test)]
mod tests {
    use crate::finite_field::FiniteField;

    #[test]
    fn finite_field_element() {
        let fp = FiniteField::new(11.into()).unwrap();
        let tests = [
            fp.elem(5.into()),
            fp.elem(30.into()),
            fp.elem((-5).into()),
            fp.elem((-30).into()),
        ];
        let res = [
            fp.elem(5.into()),
            fp.elem(8.into()),
            fp.elem(6.into()),
            fp.elem(3.into()),
        ];

        for (t, r) in tests.iter().zip(&res) {
            assert_eq!(t, r);
        }
    }

    #[test]
    fn add() {
        let fp = FiniteField::new(11.into()).unwrap();
        let tests = [
            (fp.elem(3.into()), fp.elem(4.into())),
            (fp.elem(5.into()), fp.elem(100.into())),
        ];
        let res = [fp.elem(7.into()), fp.elem(6.into())];

        for (t, r) in tests.into_iter().zip(res) {
            assert_eq!(t.0 + t.1, r);
        }
    }

    #[test]
    #[should_panic]
    fn add_err() {
        let fp1 = FiniteField::new(11.into()).unwrap();
        let fp2 = FiniteField::new(13.into()).unwrap();

        let _ = fp1.elem(3.into()) + fp2.elem(3.into());
    }

    #[test]
    fn sub() {
        let fp = FiniteField::new(11.into()).unwrap();
        let tests = [
            (fp.elem(3.into()), fp.elem(3.into())),
            (fp.elem(10.into()), fp.elem(5.into())),
            (fp.elem(100.into()), fp.elem(99.into())),
            (fp.elem(3.into()), fp.elem((-20).into())),
        ];
        let res = [
            fp.elem(0.into()),
            fp.elem(5.into()),
            fp.elem(1.into()),
            fp.elem(1.into()),
        ];

        for (t, r) in tests.into_iter().zip(res) {
            assert_eq!(t.0 - t.1, r);
        }
    }

    #[test]
    #[should_panic]
    fn sub_err() {
        let fp1 = FiniteField::new(11.into()).unwrap();
        let fp2 = FiniteField::new(13.into()).unwrap();

        let _ = fp1.elem(3.into()) - fp2.elem(3.into());
    }

    #[test]
    fn mul() {
        let fp = FiniteField::new(11.into()).unwrap();
        let tests = [
            (fp.elem(3.into()), fp.elem(3.into())),
            (fp.elem(10.into()), fp.elem(5.into())),
            (fp.elem(100.into()), fp.elem(99.into())),
            (fp.elem(3.into()), fp.elem((-20).into())),
        ];
        let res = [
            fp.elem(9.into()),
            fp.elem(6.into()),
            fp.elem(0.into()),
            fp.elem(6.into()),
        ];

        for (t, r) in tests.into_iter().zip(res) {
            assert_eq!(t.0 * t.1, r);
        }
    }

    #[test]
    #[should_panic]
    fn mul_err() {
        let fp1 = FiniteField::new(11.into()).unwrap();
        let fp2 = FiniteField::new(13.into()).unwrap();

        let _ = fp1.elem(3.into()) * fp2.elem(3.into());
    }
}
