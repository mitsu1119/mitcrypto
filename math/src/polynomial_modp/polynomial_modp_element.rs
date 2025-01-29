use std::fmt::Display;

use crate::finite_field::finite_field_element::FiniteFieldElement;

use super::polynomial_modp::PolynomialModp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolynomialModpElement<'a> {
    parent: &'a PolynomialModp,

    // f = sum^d_i coeffs[i] * X^i の形式で格納
    // e.g. x^2 + 2 -> coeffs == vec![2, 0, 1]
    // ただし 0 は vec![] で格納
    // また最高時の係数は非ゼロであることを保障
    // i.e. coeffs == vec![1, 0, 1, 0] は不正，vec![1, 0, 1] であるべき
    coeffs: Vec<FiniteFieldElement<'a>>,
}

impl<'a> PolynomialModpElement<'a> {
    pub fn new(parent: &'a PolynomialModp, coeffs: Vec<FiniteFieldElement<'a>>) -> Self {
        Self { parent, coeffs }
    }
}

impl<'a> Display for PolynomialModpElement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn term_write(
            f: &mut std::fmt::Formatter<'_>,
            coeff: &FiniteFieldElement,
            deg: usize,
            is_first: bool,
        ) -> std::fmt::Result {
            if coeff != &coeff.parent().zero() {
                if !is_first {
                    write!(f, " + ")?;
                }

                if deg == 0 {
                    write!(f, "{}", coeff)
                } else {
                    if coeff == &coeff.parent().one() {
                        write!(f, "x^{}", deg)
                    } else {
                        write!(f, "{}*x^{}", coeff, deg)
                    }
                }
            } else {
                Ok(())
            }
        }

        if self.coeffs.is_empty() {
            write!(f, "0")
        } else if self.coeffs.len() == 1 {
            term_write(f, &self.coeffs[0], 0, true)
        } else {
            term_write(f, &self.coeffs.last().unwrap(), self.coeffs.len() - 1, true)?;
            for (i, coeff) in self.coeffs[1..self.coeffs.len() - 1]
                .iter()
                .enumerate()
                .rev()
            {
                term_write(f, coeff, i + 1, false)?;
            }

            term_write(f, &self.coeffs[0], 0, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::{
        finite_field::finite_field::FiniteField, polynomial_modp::polynomial_modp::PolynomialModp,
    };

    #[test]
    fn polynomial_modp_element() {
        let f = FiniteField::new(Integer::from(13)).unwrap();
        let fx = PolynomialModp::new(f);

        let tests = [
            fx.elem(vec![
                fx.base().elem(Integer::from(1)),
                fx.base().elem(Integer::from(0)),
                fx.base().elem(Integer::from(3)),
                fx.base().elem(Integer::from(2)),
            ]),
            fx.elem(vec![fx.base().elem(Integer::from(1))]),
            fx.elem(vec![
                fx.base().elem(Integer::from(1)),
                fx.base().elem(Integer::from(0)),
                fx.base().elem(Integer::from(1)),
                fx.base().elem(Integer::from(1)),
            ]),
            fx.elem(vec![
                fx.base().elem(Integer::from(0)),
                fx.base().elem(Integer::from(0)),
                fx.base().elem(Integer::from(5)),
            ]),
            fx.elem(vec![]),
        ];

        let res = ["2*x^3 + 3*x^2 + 1", "1", "x^3 + x^2 + 1", "5*x^2", "0"];

        for (t, r) in tests.iter().zip(res) {
            assert_eq!(format!("{}", t), r);
        }
    }
}
