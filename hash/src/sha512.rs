use std::ops::Shl;

use crate::digest::HashDigest;

pub struct Sha512Digest {
    data: <Sha512Digest as HashDigest>::Digest,
}

impl Sha512Digest {
    fn new(data: <Sha512Digest as HashDigest>::Digest) -> Self {
        Self { data }
    }
}

impl HashDigest for Sha512Digest {
    type Digest = [u64; 8];

    fn digest(&self) -> Self::Digest {
        self.data
    }

    fn hexdigest(&self) -> String {
        let mut res = String::new();
        for i in self.data {
            res.push_str(&format!("{:0>16x?}", i));
        }
        res
    }
}

pub struct Sha512 {}

impl Sha512 {
    // bytes
    const BLOCK_SIZE: usize = 128;
    const WORD_SIZE: usize = 8;
    const DIGEST_SIZE: usize = 64;
    const IV: [u64; Self::DIGEST_SIZE / Self::WORD_SIZE] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    fn pad(m: Vec<u8>) -> Vec<u8> {
        let l = m.len();
        let mods = (l + 1) % Self::BLOCK_SIZE;
        let k = if mods > 112 {
            Self::BLOCK_SIZE - (mods - 112)
        } else if mods < 112 {
            112 - mods
        } else {
            0
        };
        let res = {
            let mut res = m;
            let l: u128 = l as u128;
            res.push(0b10000000);
            res.extend(vec![0x0 as u8; k]);
            for i in 1..=16 {
                res.push((((l * 8) & ((0xff) << (8 * (16 - i)))) >> (128 - 8 * i)) as u8);
            }
            res
        };
        res
    }

    fn parse(m: Vec<u8>) -> Result<Vec<[u8; Self::BLOCK_SIZE]>, Vec<u8>> {
        if m.len() % Self::BLOCK_SIZE != 0 {
            return Err(m);
        }
        let mut res = vec![];
        for block in m.chunks(Self::BLOCK_SIZE) {
            res.push(block.try_into().unwrap());
        }
        Ok(res)
    }

    pub fn hash(m: Vec<u8>) -> Result<Sha512Digest, Vec<u8>> {
        let m = Self::parse(Self::pad(m)).unwrap();

        println!("{:?}", m);

        Ok(Sha512Digest::new(Self::IV))
    }
}

#[cfg(test)]
mod tests {
    use super::Sha512;

    #[test]
    fn sha512() {
        Sha512::hash(b"test".to_vec()).unwrap();

        panic!();
    }
}
