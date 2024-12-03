use crate::digest::HashDigest;

pub struct Sha256Digest {
    data: <Sha256Digest as HashDigest>::Digest,
}

impl Sha256Digest {
    fn new(data: <Sha256Digest as HashDigest>::Digest) -> Self {
        Self { data }
    }
}

impl HashDigest for Sha256Digest {
    type Digest = [u32; 8];

    fn digest(&self) -> Self::Digest {
        self.data
    }

    fn hexdigest(&self) -> String {
        format!("{:0>8x?}", self.data)
    }
}

pub struct Sha256 {}

impl Sha256 {
    // bytes
    const BLOCK_SIZE: usize = 64;
    const WORD_SIZE: usize = 4;
    const DIGEST_SIZE: usize = 32;
    const IV: [u32; Self::DIGEST_SIZE / Self::WORD_SIZE] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    fn pad(m: Vec<u8>) -> Vec<u8> {
        let l = m.len();
        let mods = (l + 1) % Self::BLOCK_SIZE;
        let k = if mods > 56 {
            Self::BLOCK_SIZE - (mods - 56)
        } else if mods < 56 {
            56 - mods
        } else {
            0
        };
        let res = {
            let mut res = m;
            res.push(0b10000000);
            res.extend(vec![0x0 as u8; k]);
            for i in 1..9 {
                res.push((((l * 8) & ((0xff) << (8 * (8 - i)))) >> (64 - 8 * i)) as u8);
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

    pub fn hash(m: Vec<u8>) -> Result<Sha256Digest, Vec<u8>> {
        let m = Self::parse(Self::pad(m)).unwrap();

        println!("{:x?}", m);

        let hs = Self::IV;
        for block in m {}

        Ok(Sha256Digest::new(hs))
    }
}

#[cfg(test)]
mod tests {
    use super::Sha256;

    #[test]
    fn sha256() {
        Sha256::hash(vec![]).unwrap();
        panic!();
    }
}
