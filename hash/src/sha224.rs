use crate::{digest::HashDigest, sha256::Sha256};

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

    pub fn hash(m: Vec<u8>) -> Result<Sha224Digest, Vec<u8>> {
        let sha256 = Sha256::hash_iv(m, Self::IV)?;

        Ok(Sha224Digest::new(sha256.digest()[0..7].try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{digest::HashDigest, sha224::Sha224};

    #[test]
    fn sha224() {}
}