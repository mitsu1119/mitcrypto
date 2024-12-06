use crate::{digest::HashDigest, sha512::Sha512};

pub struct Sha512_224Digest {
    data: <Sha512_224Digest as HashDigest>::Digest,
}

impl Sha512_224Digest {
    fn new(data: <Sha512_224Digest as HashDigest>::Digest) -> Self {
        Self { data }
    }
}

impl HashDigest for Sha512_224Digest {
    type Digest = [u32; 7];

    fn digest(&self) -> Self::Digest {
        self.data
    }

    fn hexdigest(&self) -> String {
        let mut res = String::new();
        for i in self.data {
            res.push_str(&format!("{:0>8x?}", i));
        }
        res
    }
}

pub struct Sha512_224 {}

impl Sha512_224 {
    // bytes
    const BLOCK_SIZE: usize = 128;
    const WORD_SIZE: usize = 8;
    const DIGEST_SIZE: usize = 32;
    const IV: [u64; 8] = [
        0x8C3D37C819544DA2,
        0x73E1996689DCD4D6,
        0x1DFAB7AE32FF9C82,
        0x679DD514582F9FCF,
        0x0F6D2B697BD44DA8,
        0x77E36F7304C48942,
        0x3F9D85A86A1D36C8,
        0x1112E6AD91D692A1,
    ];

    pub fn hash(m: Vec<u8>) -> Result<Sha512_224Digest, Vec<u8>> {
        let sha512 = Sha512::hash_iv(m, Self::IV)?.digest();
        let res = [
            (sha512[0] >> 32) as u32,
            (sha512[0] & 0xffffffff) as u32,
            (sha512[1] >> 32) as u32,
            (sha512[1] & 0xffffffff) as u32,
            (sha512[2] >> 32) as u32,
            (sha512[2] & 0xffffffff) as u32,
            (sha512[3] >> 32) as u32,
        ];

        Ok(Sha512_224Digest::new(res))
    }
}

#[cfg(test)]
mod tests {
    use crate::{digest::HashDigest, sha512_224::Sha512_224};
    use cavp_tester::cavp_test::CavpTest;

    #[tokio::test]
    async fn sha512_224() {
        // NIST CAVP Testing (https://csrc.nist.gov/Projects/Cryptographic-Algorithm-Validation-Program/Secure-Hashing#shavs)
        let test = CavpTest::new("test").unwrap();
        test.download(cavp_tester::cavp_test::TestKind::SHA)
            .await
            .ok();

        for t in test.sha512_224_byte_testvectors().unwrap() {
            let md = if t.bit_len == 0 {
                Sha512_224::hash(vec![]).unwrap().hexdigest()
            } else {
                Sha512_224::hash(t.as_bytes().msg).unwrap().hexdigest()
            };

            assert!(t.test(md.trim().to_string()).is_ok());
        }
    }
}
