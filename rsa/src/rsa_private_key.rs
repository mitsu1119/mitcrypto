use mitcrypto::math::{
    error::MathError,
    mod_ring::{mod_ring::Zmod, mod_ring_element::ZmodElement},
};
use rug::{integer::IsPrime, Complete, Integer};

type Result<T> = std::result::Result<T, MathError>;

pub struct RsaPrivateKey<'a> {
    ring: Zmod,
    p: Integer,
    q: Integer,
    d: ZmodElement<'a>,
}

impl<'a> RsaPrivateKey<'a> {
    pub fn new(p: Integer, q: Integer, d: ZmodElement<'a>) -> Result<Self> {
        if p.is_probably_prime(100) == IsPrime::No || q.is_probably_prime(100) == IsPrime::No {
            Err(MathError::ValueError(
                "parameter of rsa (p, q) must be a prime".into(),
            ))
        } else {
            let n = (&p * &q).complete();
            Ok(Self {
                ring: Zmod::new(n)?,
                p,
                q,
                d,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use mitcrypto::math::mod_ring::mod_ring::Zmod;
    use rug::Integer;

    use super::RsaPrivateKey;

    #[test]
    fn rsa_private_key() {
        let pqs = [(Integer::from(3), Integer::from(5)), (Integer::from_str_radix("f271ff85ae050e6d5200e0bf2dab52e1267c02794fb775dceac60a5021c305a7bea422967abd785802945de02ec23ebf0fb1505895039c089d11b3786ebd5da5", 16).unwrap(), Integer::from_str_radix("eb563f222f6b00884811a3ba265c24bea54215ca6ffcab75adbcf81965fd3c599c41962ae2b906423ae2092707951cff1b2610057cfff85c313f67aa688e6121", 16).unwrap())];
        let ds = [Integer::from(1), Integer::from_str_radix("2cf2aecbfdb5f4665d17c9550db90a1bc42f4d4de26ad291c4fe70fab93c1420aed4fa2bc60da12464de1a53bf87b033223e2ce5eabae8dbcd981b143eb37cce4500d0386a4f563c023ef8614eec255ce2cb80a1162b91b9af9a17820fe701d16d011a5678ed130de490709c3de8ac7f15aa0ae8f78d30096f39a6445906a881", 16).unwrap()];
        let rings = pqs.clone().map(|(p, q)| Zmod::new(p * q).unwrap());
        let private_keys = pqs
            .into_iter()
            .zip(&rings)
            .zip(ds)
            .map(|(((p, q), r), d)| RsaPrivateKey::new(p, q, r.elem(d)));

        let ns = [
            Integer::from(15),
            Integer::from_str_radix("dee053a5fb953395f6dc599418ad8afcf6f9b1f7474f9b3718a92b14703d4f54ae32ba3a61eb06f434583ac33c983c2d0f9423055ba695b288650aadda1200eeaaff912221e2094091b45b8884f6cbcd6ea9a63b3df1aecc78c7f2203a01550dab994c5bb3cd12d5312182f3bcb3cb89373381a9b71952b4c91bb223006a9745", 16).unwrap()
        ];

        for (pk, n) in private_keys.zip(ns) {
            let r = pk.unwrap().ring;
            let rr = Zmod::new(Integer::from(n)).unwrap();
            assert_eq!(r, rr);
        }
    }
}
