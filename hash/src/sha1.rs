use std::ops::Shl;

use crate::digest::HashDigest;

pub struct Sha1Digest {
    data: <Sha1Digest as HashDigest>::Digest,
}

impl Sha1Digest {
    fn new(data: <Sha1Digest as HashDigest>::Digest) -> Self {
        Self { data }
    }
}

impl HashDigest for Sha1Digest {
    type Digest = [u32; 5];

    fn digest(&self) -> Self::Digest {
        self.data
    }

    fn hexdigest(&self) -> String {
        format!(
            "{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}{:0>8x?}",
            self.data[0], self.data[1], self.data[2], self.data[3], self.data[4]
        )
    }
}

pub struct Sha1 {}

impl Sha1 {
    // bytes
    const BLOCK_SIZE: usize = 64;
    const WORD_SIZE: usize = 4;
    pub const DIGEST_SIZE: usize = 20;
    const IV: [u32; Self::DIGEST_SIZE / Self::WORD_SIZE] =
        [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];
    const K: [u32; 4] = [0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xca62c1d6];

    fn ft(x: u32, y: u32, z: u32, t: usize) -> u32 {
        if t < 20 {
            (x & y) ^ ((!x) & z)
        } else if 40 <= t && t < 60 {
            (x & y) ^ (x & z) ^ (y & z)
        } else {
            x ^ y ^ z
        }
    }

    fn kt(t: usize) -> u32 {
        if t < 20 {
            Self::K[0]
        } else if t < 40 {
            Self::K[1]
        } else if t < 60 {
            Self::K[2]
        } else {
            Self::K[3]
        }
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

    pub fn hash(m: Vec<u8>) -> Result<Sha1Digest, Vec<u8>> {
        let m = Self::parse(Self::pad(m)).unwrap();

        let mut hs = Self::IV;
        for block in m {
            let mut a = hs[0];
            let mut b = hs[1];
            let mut c = hs[2];
            let mut d = hs[3];
            let mut e = hs[4];
            let wts: [u32; 80] = {
                let mut wts = [0; 80];
                for t in 0..80 {
                    if t < 16 {
                        wts[t] = (block[4 * t] as u32).shl(24)
                            | (block[4 * t + 1] as u32).shl(16)
                            | (block[4 * t + 2] as u32).shl(8)
                            | (block[4 * t + 3] as u32);
                    } else {
                        wts[t] = (wts[t - 3] ^ wts[t - 8] ^ wts[t - 14] ^ wts[t - 16] as u32)
                            .rotate_left(1);
                    };
                }
                wts
            };
            for t in 0..80 {
                let tt = a
                    .rotate_left(5)
                    .wrapping_add(Self::ft(b, c, d, t))
                    .wrapping_add(e)
                    .wrapping_add(Self::kt(t))
                    .wrapping_add(wts[t]);

                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = tt;
            }
            hs[0] = hs[0].wrapping_add(a);
            hs[1] = hs[1].wrapping_add(b);
            hs[2] = hs[2].wrapping_add(c);
            hs[3] = hs[3].wrapping_add(d);
            hs[4] = hs[4].wrapping_add(e);
        }

        Ok(Sha1Digest::new(hs))
    }
}

#[cfg(test)]
mod tests {
    use crate::{digest::HashDigest, sha1::Sha1};
    use cavp_tester::cavp_test::CavpTest;

    #[tokio::test]
    async fn sha1() {
        // NIST CAVP Testing (https://csrc.nist.gov/Projects/Cryptographic-Algorithm-Validation-Program/Secure-Hashing#shavs)
        let test = CavpTest::new("test").unwrap();
        test.download(cavp_tester::cavp_test::TestKind::SHA)
            .await
            .ok();

        for t in test.sha1_byte_testvectors().unwrap() {
            let md = if t.bit_len == 0 {
                Sha1::hash(vec![]).unwrap().hexdigest()
            } else {
                Sha1::hash(t.as_bytes().msg).unwrap().hexdigest()
            };

            assert!(t.test(md.trim().to_string()).is_ok());
        }
    }
}
