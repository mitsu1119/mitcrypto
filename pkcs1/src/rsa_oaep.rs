use hash::{digest::HashDigest, sha1::Sha1};
use rug::{integer::IntegerExt64, Integer};

use crate::{
    error::Pkcs1Error,
    rsa_private_key::RsaPrivateKey,
    rsa_public_key::RsaPublicKey,
    util::{i2osp, os2ip},
};

type Result<T> = std::result::Result<T, Pkcs1Error>;

struct RsaOaep {}

impl RsaOaep {
    fn mgf1(mgf_seed: Vec<u8>, mask_len: usize) -> Result<Vec<u8>> {
        if mask_len > Sha1::DIGEST_SIZE << 32 {
            return Err(Pkcs1Error::ValueError("mask_len too large".into()));
        }

        let mut t: Vec<u8> = vec![];
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
            t.extend(res);
        }

        Ok(t[..mask_len].to_vec())
    }

    pub fn encrypt_with_seed(
        m: Vec<u8>,
        pubkey: RsaPublicKey,
        l: Vec<u8>,
        seed: Vec<u8>,
    ) -> Result<Vec<u8>> {
        let n = pubkey.ring.order();

        if m.len() > n.significant_bits().div_ceil(8) as usize - 2 * Sha1::DIGEST_SIZE - 2 {
            return Err(Pkcs1Error::ValueError("message too long".into()));
        }

        if l.len() > Sha1::MAX_MESSAGE_SIZE {
            return Err(Pkcs1Error::ValueError("label too long".into()));
        }

        if seed.len() != Sha1::DIGEST_SIZE {
            return Err(Pkcs1Error::ValueError("seed len is invalid".into()));
        }

        let lhash: Vec<u8> = Sha1::hash(l)
            .unwrap()
            .digest()
            .map(|x| i2osp(Integer::from(x), 4).unwrap())
            .into_iter()
            .flatten()
            .collect();
        println!("lhash: {:x?}", lhash);
        println!("len: {}", lhash.len());

        let k = n.significant_bits().div_ceil(8);
        let ps: Vec<u8> = vec![0; k as usize - m.len() - 2 * Sha1::DIGEST_SIZE - 2];
        let db = [lhash, ps, vec![1], m].concat();

        println!("db: {:x?}", db);
        println!("len: {}", db.len());
        println!("seed: {:x?}", seed);

        let masked_db: Vec<u8> = Self::mgf1(seed.clone(), k as usize - Sha1::DIGEST_SIZE - 1)?
            .into_iter()
            .zip(db.into_iter())
            .map(|(x, y)| x ^ y)
            .collect();
        println!("masked_db: {:x?}", masked_db);
        let masked_seed: Vec<u8> = Self::mgf1(masked_db.clone(), Sha1::DIGEST_SIZE)?
            .into_iter()
            .zip(seed.into_iter())
            .map(|(x, y)| x ^ y)
            .collect();
        println!("masked_seed: {:x?}", masked_seed);

        let em = [vec![0], masked_seed, masked_db].concat();
        println!("em: {:x?}", em);
        println!("len: {}", em.len());
        println!("k: {}", k);
        let m = os2ip(em);
        let c = pubkey.encrypt(m).unwrap();
        let len = c.as_integer().significant_bits().div_ceil(8) as usize;

        Ok(i2osp(c.as_integer().clone(), len).unwrap())
    }

    pub fn decrypt(
        c: Vec<u8>,
        pubkey: RsaPublicKey,
        private_key: RsaPrivateKey,
        label: Vec<u8>,
    ) -> Result<Vec<u8>> {
        let k = pubkey.ring.order().significant_bits().div_ceil(8) as usize;
        let c_len = c.len();
        let h_len = Sha1::DIGEST_SIZE;

        if k < 2 * h_len + 2 || c_len != k || label.len() > Sha1::MAX_MESSAGE_SIZE {
            return Err(Pkcs1Error::ValueError("decryption error".into()));
        }

        let c = os2ip(c);
        let m = private_key.decrypt(c).unwrap().as_integer().clone();
        let em = i2osp(m, k).unwrap();

        let l_hash = Sha1::hash(label).unwrap().digest_u8();

        let masked_db = &em[h_len + 1..];
        let masked_seed = &em[1..h_len + 1];
        let y = em[0];
        println!("masked_seed: {:x?}", masked_seed);
        println!("masked_db: {:x?}", masked_db);

        let seed: Vec<u8> = Self::mgf1(masked_db.to_vec(), h_len)
            .unwrap()
            .into_iter()
            .zip(masked_seed)
            .map(|(x, y)| x ^ y)
            .collect();

        println!("seed: {:x?}", seed);

        println!("{} {}", k - h_len - 1, masked_db.len());
        let db: Vec<u8> = Self::mgf1(seed, k - h_len - 1)
            .unwrap()
            .into_iter()
            .zip(masked_db)
            .map(|(x, y)| x ^ y)
            .collect();

        let db_lhash = &db[..h_len];
        if y != 0 || db_lhash.to_vec() != l_hash {
            return Err(Pkcs1Error::ValueError("decryption error".into()));
        }

        let ps_end = (&db[h_len..]).iter().position(|x| *x == 1).unwrap();

        Ok((&db[h_len + ps_end + 1..]).to_vec())
    }
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::{rsa_oaep::RsaOaep, rsa_private_key::RsaPrivateKey, rsa_public_key::RsaPublicKey};

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

    #[test]
    fn encrypt() {
        let p = Integer::from_str_radix("eabd3a31df44c8dbe59063925b1ad83555abdf34e056f8cc88be97a8bf386e163e076523aabd54c7afc17d07f6f77ddfa1af5923cb1cd9272cfc76afbd422bb3", 16).unwrap();
        let q = Integer::from_str_radix("f3e18cf05b052e567137ec63081286636b5bc910da4421976817c0fbffe0ec139e1d87414d121e1606138e6b84f62fcfab5a8bc738bf0c2c9ce609803ea83715", 16).unwrap();
        let n = Integer::from_str_radix("dfa06fc95d735ed753af82d3d68573179c2688901158b091d7ead83e693a1d4029e2400a929236294135307886d042ade385a7f29f9cdfccb6d6a8dde980a6f143341fc93eabb5e276df1d9f709402fdcb3b2c4b26e8a7716f51574a115621d6de7088afac07aa4090920b5e95a366c12e73f9905638adabacd0dc95c5490aaf", 16).unwrap();
        let e = Integer::from(0x10001);
        let d = Integer::from_str_radix("39c93479bd4b3dbbb8a546d244c1d13ecd9beb7806f37b1504cd6bb99ce9667b99171ce35e82a7ba3b5e6a3b0ae33007cd1e518ad191f106ee4c43f0ac11119214d4bec6d1cc0c4168d95ba5d0b6b04e67b76e2b149469998e106c52fe1034acd146068a0b4de78a04ea07db919632cf9b071574aef6c8d26512a10fb05c6459", 16).unwrap();
        let pubkey = RsaPublicKey::new(n, e).unwrap();
        let private_key = RsaPrivateKey::new(p, q, d).unwrap();

        let label: Vec<u8> = vec![];
        let seed = vec![
            0x18, 0xb7, 0x76, 0xea, 0x21, 0x06, 0x9d, 0x69, 0x77, 0x6a, 0x33, 0xe9, 0x6b, 0xad,
            0x48, 0xe1, 0xdd, 0xa0, 0xa5, 0xef,
        ];
        let m = vec![
            0x66, 0x28, 0x19, 0x4e, 0x12, 0x07, 0x3d, 0xb0, 0x3b, 0xa9, 0x4c, 0xda, 0x9e, 0xf9,
            0x53, 0x23, 0x97, 0xd5, 0x0d, 0xba, 0x79, 0xb9, 0x87, 0x00, 0x4a, 0xfe, 0xfe, 0x34,
        ];

        let res = RsaOaep::encrypt_with_seed(m, pubkey.clone(), label.clone(), seed).unwrap();

        println!("");

        let m = RsaOaep::decrypt(res, pubkey, private_key, label).unwrap();
        println!("m: {:x?}", m);
        panic!();
    }
}
