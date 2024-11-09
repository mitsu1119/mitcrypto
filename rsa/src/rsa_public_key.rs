use mitcrypto::math::{error::MathError, finite_field::FiniteField};
use rug::Integer;

type Result<T> = std::result::Result<T, MathError>;

pub struct RsaPublicKey {
    field: FiniteField,
    e: Integer,
}

impl RsaPublicKey {
    pub fn new(p: Integer, e: Integer) -> Result<Self> {
        Ok(Self {
            field: FiniteField::new(p)?,
            e,
        })
    }
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::rsa_public_key::RsaPublicKey;

    #[test]
    fn rsa() {
        assert!(RsaPublicKey::new(11.into(), 3.into()).is_ok());
        assert!(RsaPublicKey::new(Integer::from_str_radix("99d1ef082468dab6f24206cfe2dca547beb8055bd9714743aa56ff0c6d6fc09c00be6ed6fbcee6cba68d6c5785456f050e091e9237d9f8e01e09ae6fcbddae3f95c4e397ac762fde619367732647c2d34ca9b8e3973213f6904634d4689430f7cc1f385e2e361b8f00ca5a27a7b8d70ba11d12993381d1a4687b60c839638ac5", 16).unwrap(), 0x1001.into()).is_ok());
        assert!(RsaPublicKey::new(4.into(), 3.into()).is_err());
        assert!(RsaPublicKey::new(Integer::from_str_radix("99d1ef082468dab6f24206cfe2dca547beb8055bd9714743aa56ff0c6d6fc09c00be6ed6fbcee6cba68d6c5785456f050e091e9237d9f8e01e09ae6fcbddae3f95c4e397ac762fde619367732647c2d34ca9b8e3973213f6904634d4689430f7cc1f385e2e361b8f00ca5a27a7b8d70ba11d12993381d1a4687b60c839638ac6", 16).unwrap(), 0x1001.into()).is_err());
    }
}
