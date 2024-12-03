use std::ops::Shl;

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
        format!(
            "{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}",
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
            self.data[4],
            self.data[5],
            self.data[6],
            self.data[7]
        )
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
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    #[inline(always)]
    fn kt(t: usize) -> u32 {
        Self::K[t]
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    fn S0(x: u32) -> u32 {
        x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    fn S1(x: u32) -> u32 {
        x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
    }

    #[inline(always)]
    fn s0(x: u32) -> u32 {
        x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
    }

    #[inline(always)]
    fn s1(x: u32) -> u32 {
        x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
    }

    #[inline(always)]
    fn ch(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ ((!x) & z)
    }

    #[inline(always)]
    fn maj(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (x & z) ^ (y & z)
    }

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

        let mut hs = Self::IV;
        for block in m {
            let mut tmps = hs;
            let wts: [u32; 64] = {
                let mut wts = [0; 64];
                for t in 0..64 {
                    if t < 16 {
                        wts[t] = (block[4 * t] as u32).shl(24)
                            | (block[4 * t + 1] as u32).shl(16)
                            | (block[4 * t + 2] as u32).shl(8)
                            | (block[4 * t + 3] as u32);
                    } else {
                        wts[t] = Self::s1(wts[t - 2])
                            .wrapping_add(wts[t - 7])
                            .wrapping_add(Self::s0(wts[t - 15]).wrapping_add(wts[t - 16]));
                    };
                }
                wts
            };

            for t in 0..64 {
                let t1 = tmps[7]
                    .wrapping_add(Self::S1(tmps[4]))
                    .wrapping_add(Self::ch(tmps[4], tmps[5], tmps[6]))
                    .wrapping_add(Self::kt(t))
                    .wrapping_add(wts[t]);
                let t2 = Self::S0(tmps[0]).wrapping_add(Self::maj(tmps[0], tmps[1], tmps[2]));

                tmps[7] = tmps[6];
                tmps[6] = tmps[5];
                tmps[5] = tmps[4];
                tmps[4] = tmps[3].wrapping_add(t1);
                tmps[3] = tmps[2];
                tmps[2] = tmps[1];
                tmps[1] = tmps[0];
                tmps[0] = t1.wrapping_add(t2);
            }
            for i in 0..hs.len() {
                hs[i] = hs[i].wrapping_add(tmps[i]);
            }
        }

        Ok(Sha256Digest::new(hs))
    }
}

#[cfg(test)]
mod tests {
    use cavp_tester::cavp_test::CavpTest;

    use crate::digest::HashDigest;

    use super::Sha256;

    #[tokio::test]
    async fn sha256() {
        // NIST CAVP Testing (https://csrc.nist.gov/Projects/Cryptographic-Algorithm-Validation-Program/Secure-Hashing#shavs)
        let test = CavpTest::new("test").unwrap();
        test.download(cavp_tester::cavp_test::TestKind::SHA)
            .await
            .ok();

        for t in test.sha256_byte_testvectors().unwrap() {
            let md = if t.bit_len == 0 {
                Sha256::hash(vec![]).unwrap().hexdigest()
            } else {
                Sha256::hash(t.as_bytes().msg).unwrap().hexdigest()
            };

            assert!(t.test(md.trim().to_string()).is_ok());
        }
    }
}
