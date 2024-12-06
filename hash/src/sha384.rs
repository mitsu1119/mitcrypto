use crate::{digest::HashDigest, sha512::Sha512};

pub struct Sha384Digest {
    data: <Sha384Digest as HashDigest>::Digest,
}

impl Sha384Digest {
    fn new(data: <Sha384Digest as HashDigest>::Digest) -> Self {
        Self { data }
    }
}

impl HashDigest for Sha384Digest {
    type Digest = [u64; 6];

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

pub struct Sha384 {}

impl Sha384 {
    // bytes
    const BLOCK_SIZE: usize = 128;
    const WORD_SIZE: usize = 8;
    const DIGEST_SIZE: usize = 48;
    const IV: [u64; 8] = [
        0xcbbb9d5dc1059ed8,
        0x629a292a367cd507,
        0x9159015a3070dd17,
        0x152fecd8f70e5939,
        0x67332667ffc00b31,
        0x8eb44a8768581511,
        0xdb0c2e0d64f98fa7,
        0x47b5481dbefa4fa4,
    ];

    pub fn hash(m: Vec<u8>) -> Result<Sha384Digest, Vec<u8>> {
        let sha512 = Sha512::hash_iv(m, Self::IV)?;

        Ok(Sha384Digest::new(sha512.digest()[0..6].try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{digest::HashDigest, sha384::Sha384};
    use cavp_tester::cavp_test::CavpTest;

    #[tokio::test]
    async fn sha384() {
        // NIST CAVP Testing (https://csrc.nist.gov/Projects/Cryptographic-Algorithm-Validation-Program/Secure-Hashing#shavs)
        let test = CavpTest::new("test").unwrap();
        test.download(cavp_tester::cavp_test::TestKind::SHA)
            .await
            .ok();

        for t in test.sha384_byte_testvectors().unwrap() {
            let md = if t.bit_len == 0 {
                Sha384::hash(vec![]).unwrap().hexdigest()
            } else {
                Sha384::hash(t.as_bytes().msg).unwrap().hexdigest()
            };

            assert!(t.test(md.trim().to_string()).is_ok());
        }
    }
}
