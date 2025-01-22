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
        // The test vector is taken from (https://github.com/pyca/cryptography/blob/main/vectors/cryptography_vectors/asymmetric/RSA/pkcs-1v2-1d2-vec/oaep-vect.txt)

        let n = Integer::from_str_radix("a8b3b284af8eb50b387034a860f146c4919f318763cd6c5598c8ae4811a1e0abc4c7e0b082d693a5e7fced675cf4668512772c0cbc64a742c6c630f533c8cc72f62ae833c40bf25842e984bb78bdbf97c0107d55bdb662f5c4e0fab9845cb5148ef7392dd3aaff93ae1e6b667bb3d4247616d4f5ba10d4cfd226de88d39f16fb", 16).unwrap();
        let e = Integer::from(0x10001);
        let d = Integer::from_str_radix("53339cfdb79fc8466a655c7316aca85c55fd8f6dd898fdaf119517ef4f52e8fd8e258df93fee180fa0e4ab29693cd83b152a553d4ac4d1812b8b9fa5af0e7f55fe7304df41570926f3311f15c4d65a732c483116ee3d3d2d0af3549ad9bf7cbfb78ad884f84d5beb04724dc7369b31def37d0cf539e9cfcdd3de653729ead5d1 ", 16).unwrap();
        let p = Integer::from_str_radix("d32737e7267ffe1341b2d5c0d150a81b586fb3132bed2f8d5262864a9cb9f30af38be448598d413a172efb802c21acf1c11c520c2f26a471dcad212eac7ca39d", 16).unwrap();
        let q = Integer::from_str_radix("cc8853d1d54da630fac004f471f281c7b8982d8224a490edbeb33d3e3d5cc93c4765703d1dd791642f1f116a0dd852be2419b2af72bfe9a030e860b0288b5d77", 16).unwrap();
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

        let test = RsaOaep::encrypt_with_seed(m, pubkey.clone(), label.clone(), seed).unwrap();

        let res = vec![
            0x35, 0x4f, 0xe6, 0x7b, 0x4a, 0x12, 0x6d, 0x5d, 0x35, 0xfe, 0x36, 0xc7, 0x77, 0x79,
            0x1a, 0x3f, 0x7b, 0xa1, 0x3d, 0xef, 0x48, 0x4e, 0x2d, 0x39, 0x08, 0xaf, 0xf7, 0x22,
            0xfa, 0xd4, 0x68, 0xfb, 0x21, 0x69, 0x6d, 0xe9, 0x5d, 0x0b, 0xe9, 0x11, 0xc2, 0xd3,
            0x17, 0x4f, 0x8a, 0xfc, 0xc2, 0x01, 0x03, 0x5f, 0x7b, 0x6d, 0x8e, 0x69, 0x40, 0x2d,
            0xe5, 0x45, 0x16, 0x18, 0xc2, 0x1a, 0x53, 0x5f, 0xa9, 0xd7, 0xbf, 0xc5, 0xb8, 0xdd,
            0x9f, 0xc2, 0x43, 0xf8, 0xcf, 0x92, 0x7d, 0xb3, 0x13, 0x22, 0xd6, 0xe8, 0x81, 0xea,
            0xa9, 0x1a, 0x99, 0x61, 0x70, 0xe6, 0x57, 0xa0, 0x5a, 0x26, 0x64, 0x26, 0xd9, 0x8c,
            0x88, 0x00, 0x3f, 0x84, 0x77, 0xc1, 0x22, 0x70, 0x94, 0xa0, 0xd9, 0xfa, 0x1e, 0x8c,
            0x40, 0x24, 0x30, 0x9c, 0xe1, 0xec, 0xcc, 0xb5, 0x21, 0x00, 0x35, 0xd4, 0x7a, 0xc7,
            0x2e, 0x8a,
        ];

        assert_eq!(test, res);
    }
}
