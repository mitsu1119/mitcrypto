use crate::{digest::HashDigest, sha512::Sha512};

pub struct Sha512_256Digest {
    data: <Sha512_256Digest as HashDigest>::Digest,
}

impl Sha512_256Digest {
    fn new(data: <Sha512_256Digest as HashDigest>::Digest) -> Self {
        Self { data }
    }
}

impl HashDigest for Sha512_256Digest {
    type Digest = [u64; 4];

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

pub struct Sha512_256 {}

impl Sha512_256 {
    // bytes
    const BLOCK_SIZE: usize = 128;
    const WORD_SIZE: usize = 8;
    const DIGEST_SIZE: usize = 32;
    const IV: [u64; 8] = [
        0x22312194FC2BF72C,
        0x9F555FA3C84C64C2,
        0x2393B86B6F53B151,
        0x963877195940EABD,
        0x96283EE2A88EFFE3,
        0xBE5E1E2553863992,
        0x2B0199FC2C85B8AA,
        0x0EB72DDC81C52CA2,
    ];

    pub fn hash(m: Vec<u8>) -> Result<Sha512_256Digest, Vec<u8>> {
        let sha512 = Sha512::hash_iv(m, Self::IV)?;

        Ok(Sha512_256Digest::new(
            sha512.digest()[0..4].try_into().unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{digest::HashDigest, sha512_256::Sha512_256};
    use cavp_tester::cavp_test::CavpTest;

    #[tokio::test]
    async fn sha512_256() {
        // NIST CAVP Testing (https://csrc.nist.gov/Projects/Cryptographic-Algorithm-Validation-Program/Secure-Hashing#shavs)
        let test = CavpTest::new("test").unwrap();
        test.download(cavp_tester::cavp_test::TestKind::SHA)
            .await
            .ok();

        for t in test.sha512_256_byte_testvectors().unwrap() {
            let md = if t.bit_len == 0 {
                Sha512_256::hash(vec![]).unwrap().hexdigest()
            } else {
                Sha512_256::hash(t.as_bytes().msg).unwrap().hexdigest()
            };

            assert!(t.test(md.trim().to_string()).is_ok());
        }
    }
}
