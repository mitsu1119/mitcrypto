use std::fmt::Display;

use crate::finite_field::{finite_field::FiniteField, finite_field_element::FiniteFieldElement};

use super::polynomial_modp_element::PolynomialModpElement;

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

    pub fn elem<'a>(&'a self, coeffs: Vec<FiniteFieldElement<'a>>) -> PolynomialModpElement {
        PolynomialModpElement::new(self, coeffs)
    }
}

impl Display for PolynomialModp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Univariate polynomial ring over {}", self.base)
    }
}
