use hash::{
    digest::HashDigest,
    sha1::{self, Sha1},
};
use rug::Integer;

use crate::{error::Pkcs1Error, util::i2osp};

type Result<T> = std::result::Result<T, Pkcs1Error>;

struct RsaOaep {}

impl RsaOaep {
    fn mgf1(mgf_seed: Vec<u8>, mask_len: usize) -> Result<Vec<u8>> {
        if mask_len > Sha1::DIGEST_SIZE << 32 {
            return Err(Pkcs1Error::ValueError("mask_len too large".into()));
        }

        let mut T: Vec<u8> = vec![];
        for i in 0..mask_len.div_ceil(Sha1::DIGEST_SIZE) {
            let c = i2osp(i.into(), 4).unwrap();
            let mut seed = mgf_seed.clone();
            seed.extend(c);
            let digest = Sha1::hash(seed).unwrap().digest();
            let res: Vec<u8> = digest
                .map(|x| i2osp(Integer::from(x), 4).unwrap())
                .into_iter()
                .flatten()
                .collect();
            T.extend(res);
        }

        Ok(T[..mask_len].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::rsa_oaep::RsaOaep;

    #[test]
    fn mgf1() {
        let tests = [
            (b"\x12\x34".to_vec(), 4),
            (b"\x12\x34".to_vec(), 30),
            (b"test_dayonn".to_vec(), 50),
        ];

        let res = [b"\xc8\x80\xac\x93".to_vec(), b"\xc8\x80\xac\x93\x87\xc9_\xae;x5\x16\xb3<.\xffV\xa2A\xb1\x142U\xd3\x06\xe3\x06\xda\x183".to_vec(), b"i!l<j\x12\x07\x1d\x02PY\x07\xda\xf7\x90F\x80@\xef\xe3\x13\xd9\x06\xca\xc7\x8d\x01\x06\x94zyO\x99\xda,\xd2/ioG\x9f(Gx\x99\xde\xdc\xf2 #".to_vec()];

        for ((seed, len), y) in tests.into_iter().zip(res) {
            assert_eq!(RsaOaep::mgf1(seed, len).unwrap(), y);
        }
    }
}
