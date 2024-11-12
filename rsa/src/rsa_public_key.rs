use mitcrypto::math::{
    error::MathError,
    finite_field::finite_field::FiniteField,
    mod_ring::{mod_ring::Zmod, mod_ring_element::ZmodElement},
};
use rug::Integer;

type Result<T> = std::result::Result<T, MathError>;

pub struct RsaPublicKey {
    ring: Zmod,
    e: Integer,
}

impl RsaPublicKey {
    pub fn new(n: Integer, e: Integer) -> Result<Self> {
        Ok(Self {
            ring: Zmod::new(n)?,
            e,
        })
    }

    pub fn rsa(&self, m: Integer) -> Result<ZmodElement> {
        Ok(self.ring.elem(m).pow(&self.e)?)
    }
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::rsa_public_key::RsaPublicKey;

    #[test]
    fn rsa() {
        assert!(RsaPublicKey::new(11.into(), 3.into()).is_ok());
        assert!(RsaPublicKey::new((-11).into(), 3.into()).is_ok());
        assert!(RsaPublicKey::new(Integer::from_str_radix("7123338efc611417ec92d33a84277f3a63b765736b38924905a9ba733f28e323d2aa79c5ab09db7734d220c52e46e9a6ff3f61aff745b0f340ac59891e9487a9994d4376fd89123b9547462539bc5971ca9f56169f0142b824f473754d5372e57358a652c3162ec19d1555da491dfc747a190aec0cd198f1dc95a3908ae44ebb", 16).unwrap(), 0x1001.into()).is_ok());
        assert!(RsaPublicKey::new(0.into(), 3.into()).is_err());
    }
}
