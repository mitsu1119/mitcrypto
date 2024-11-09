use core::panic;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use rug::Integer;

use crate::error::MathError;

use super::mod_ring::Zmod;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZmodElement<'a> {
    parent: &'a Zmod,
    x: Integer,
}

impl<'a> ZmodElement<'a> {
    pub fn new(parent: &'a Zmod, x: Integer) -> Self {
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

impl<'a> AddAssign for ZmodElement<'a> {
    fn add_assign(&mut self, rhs: Self) {
        self.add(rhs);
    }
}

impl<'a> Add for ZmodElement<'a> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<'a> SubAssign for ZmodElement<'a> {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub(rhs);
    }
}

impl<'a> Sub for ZmodElement<'a> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl<'a> MulAssign for ZmodElement<'a> {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul(rhs);
    }
}

impl<'a> Mul for ZmodElement<'a> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl<'a> Display for ZmodElement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}

#[cfg(test)]
mod tests {
    use crate::mod_ring::mod_ring::Zmod;

    #[test]
    fn zmod_element() {
        let r = Zmod::new(20.into()).unwrap();
        let tests = [
            r.elem(5.into()),
            r.elem(50.into()),
            r.elem((-5).into()),
            r.elem((-55).into()),
        ];
        let res = [
            r.elem(5.into()),
            r.elem(10.into()),
            r.elem(15.into()),
            r.elem(5.into()),
        ];

        for (t, r) in tests.iter().zip(&res) {
            assert_eq!(t, r);
        }
    }

    #[test]
    fn add() {
        let r = Zmod::new(50.into()).unwrap();
        let tests = [
            (r.elem(3.into()), r.elem(4.into())),
            (r.elem(5.into()), r.elem(100.into())),
        ];
        let res = [r.elem(7.into()), r.elem(5.into())];

        for (t, r) in tests.into_iter().zip(res) {
            assert_eq!(t.0 + t.1, r);
        }
    }

    #[test]
    #[should_panic]
    fn add_err() {
        let r1 = Zmod::new(11.into()).unwrap();
        let r2 = Zmod::new(13.into()).unwrap();

        let _ = r1.elem(3.into()) + r2.elem(3.into());
    }

    #[test]
    fn sub() {
        let r = Zmod::new(11.into()).unwrap();
        let tests = [
            (r.elem(3.into()), r.elem(3.into())),
            (r.elem(10.into()), r.elem(5.into())),
            (r.elem(100.into()), r.elem(99.into())),
            (r.elem(3.into()), r.elem((-20).into())),
        ];
        let res = [
            r.elem(0.into()),
            r.elem(5.into()),
            r.elem(1.into()),
            r.elem(1.into()),
        ];

        for (t, r) in tests.into_iter().zip(res) {
            assert_eq!(t.0 - t.1, r);
        }
    }

    #[test]
    #[should_panic]
    fn sub_err() {
        let r1 = Zmod::new(11.into()).unwrap();
        let r2 = Zmod::new(13.into()).unwrap();

        let _ = r1.elem(3.into()) - r2.elem(3.into());
    }

    #[test]
    fn mul() {
        let r = Zmod::new(30.into()).unwrap();
        let tests = [
            (r.elem(3.into()), r.elem(3.into())),
            (r.elem(10.into()), r.elem(5.into())),
            (r.elem(100.into()), r.elem(99.into())),
            (r.elem(3.into()), r.elem((-20).into())),
        ];
        let res = [
            r.elem(9.into()),
            r.elem(20.into()),
            r.elem(0.into()),
            r.elem(0.into()),
        ];

        for (t, r) in tests.into_iter().zip(res) {
            assert_eq!(t.0 * t.1, r);
        }
    }

    #[test]
    #[should_panic]
    fn mul_err() {
        let r1 = Zmod::new(11.into()).unwrap();
        let r2 = Zmod::new(13.into()).unwrap();

        let _ = r1.elem(3.into()) * r2.elem(3.into());
    }
}
