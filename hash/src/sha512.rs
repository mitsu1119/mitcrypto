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
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];

    const K: [u64; 80] = [
        0x428a2f98d728ae22,
        0x7137449123ef65cd,
        0xb5c0fbcfec4d3b2f,
        0xe9b5dba58189dbbc,
        0x3956c25bf348b538,
        0x59f111f1b605d019,
        0x923f82a4af194f9b,
        0xab1c5ed5da6d8118,
        0xd807aa98a3030242,
        0x12835b0145706fbe,
        0x243185be4ee4b28c,
        0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f,
        0x80deb1fe3b1696b1,
        0x9bdc06a725c71235,
        0xc19bf174cf692694,
        0xe49b69c19ef14ad2,
        0xefbe4786384f25e3,
        0x0fc19dc68b8cd5b5,
        0x240ca1cc77ac9c65,
        0x2de92c6f592b0275,
        0x4a7484aa6ea6e483,
        0x5cb0a9dcbd41fbd4,
        0x76f988da831153b5,
        0x983e5152ee66dfab,
        0xa831c66d2db43210,
        0xb00327c898fb213f,
        0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2,
        0xd5a79147930aa725,
        0x06ca6351e003826f,
        0x142929670a0e6e70,
        0x27b70a8546d22ffc,
        0x2e1b21385c26c926,
        0x4d2c6dfc5ac42aed,
        0x53380d139d95b3df,
        0x650a73548baf63de,
        0x766a0abb3c77b2a8,
        0x81c2c92e47edaee6,
        0x92722c851482353b,
        0xa2bfe8a14cf10364,
        0xa81a664bbc423001,
        0xc24b8b70d0f89791,
        0xc76c51a30654be30,
        0xd192e819d6ef5218,
        0xd69906245565a910,
        0xf40e35855771202a,
        0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8,
        0x1e376c085141ab53,
        0x2748774cdf8eeb99,
        0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63,
        0x4ed8aa4ae3418acb,
        0x5b9cca4f7763e373,
        0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc,
        0x78a5636f43172f60,
        0x84c87814a1f0ab72,
        0x8cc702081a6439ec,
        0x90befffa23631e28,
        0xa4506cebde82bde9,
        0xbef9a3f7b2c67915,
        0xc67178f2e372532b,
        0xca273eceea26619c,
        0xd186b8c721c0c207,
        0xeada7dd6cde0eb1e,
        0xf57d4f7fee6ed178,
        0x06f067aa72176fba,
        0x0a637dc5a2c898a6,
        0x113f9804bef90dae,
        0x1b710b35131c471b,
        0x28db77f523047d84,
        0x32caab7b40c72493,
        0x3c9ebe0a15c9bebc,
        0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6,
        0x597f299cfc657e2a,
        0x5fcb6fab3ad6faec,
        0x6c44198c4a475817,
    ];

    #[inline(always)]
    fn kt(t: usize) -> u64 {
        Self::K[t]
    }

    #[inline(always)]
    fn ch(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ ((!x) & z)
    }

    #[inline(always)]
    fn maj(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ (x & z) ^ (y & z)
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    fn S0(x: u64) -> u64 {
        x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    fn S1(x: u64) -> u64 {
        x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
    }

    #[inline(always)]
    fn s0(x: u64) -> u64 {
        x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7)
    }

    #[inline(always)]
    fn s1(x: u64) -> u64 {
        x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6)
    }

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

        let mut hs = Self::IV;
        for block in m {
            let mut tmps = hs;
            let wts: [u64; 80] = {
                let mut wts = [0; 80];
                for t in 0..80 {
                    if t < 16 {
                        wts[t] = (block[8 * t] as u64).shl(56)
                            | (block[8 * t + 1] as u64).shl(48)
                            | (block[8 * t + 2] as u64).shl(40)
                            | (block[8 * t + 3] as u64).shl(32)
                            | (block[8 * t + 4] as u64).shl(24)
                            | (block[8 * t + 5] as u64).shl(16)
                            | (block[8 * t + 6] as u64).shl(8)
                            | (block[8 * t + 7] as u64);
                    } else {
                        wts[t] = Self::s1(wts[t - 2])
                            .wrapping_add(wts[t - 7])
                            .wrapping_add(Self::s0(wts[t - 15]).wrapping_add(wts[t - 16]));
                    };
                }
                wts
            };

            for t in 0..80 {
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

        Ok(Sha512Digest::new(hs))
    }
}

#[cfg(test)]
mod tests {
    use crate::digest::HashDigest;

    use super::Sha512;

    #[test]
    fn sha512() {
        panic!();
    }
}
