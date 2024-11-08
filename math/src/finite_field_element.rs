use rug::Integer;

use crate::finite_field::FiniteField;

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
}

#[cfg(test)]
mod tests {
    use crate::finite_field::FiniteField;

    use super::FiniteFieldElement;

    #[test]
    fn finite_field_element() {
        let fp = FiniteField::new(11.into()).unwrap();
        let tests = [
            FiniteFieldElement::new(&fp, 5.into()),
            FiniteFieldElement::new(&fp, 30.into()),
            FiniteFieldElement::new(&fp, (-5).into()),
            FiniteFieldElement::new(&fp, (-30).into()),
        ];
        let res = [
            FiniteFieldElement::new(&fp, 5.into()),
            FiniteFieldElement::new(&fp, 8.into()),
            FiniteFieldElement::new(&fp, 6.into()),
            FiniteFieldElement::new(&fp, 3.into()),
        ];

        for (t, r) in tests.iter().zip(&res) {
            assert_eq!(t, r);
        }
    }
}
