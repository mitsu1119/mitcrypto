use crate::digest::HashDigest;

pub struct Sha224Digest {
    data: <Sha224Digest as HashDigest>::Digest,
}

impl Sha224Digest {
    fn new(data: <Sha224Digest as HashDigest>::Digest) -> Self {
        Self { data }
    }
}

impl HashDigest for Sha224Digest {
    type Digest = [u32; 7];

    fn digest(&self) -> Self::Digest {
        self.data
    }

    fn hexdigest(&self) -> String {
        format!(
            "{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}",
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
            self.data[4],
            self.data[5],
            self.data[6],
        )
    }
}

pub struct Sha224 {}

impl Sha224 {
    // bytes
    const BLOCK_SIZE: usize = 64;
    const WORD_SIZE: usize = 4;
    const DIGEST_SIZE: usize = 28;
    const IV: [u32; 8] = [
        0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
        0xbefa4fa4,
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

    pub fn hash(m: Vec<u8>) -> Result<Sha224Digest, Vec<u8>> {
        let m = Self::parse(Self::pad(m)).unwrap();

        let mut hs = Self::IV;

        Ok(Sha224Digest::new(hs[0..7].try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sha224() {}
}
