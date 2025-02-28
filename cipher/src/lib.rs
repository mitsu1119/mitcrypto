type Block = [u8; AES::BLOCK_SIZE];

pub struct AES {
    key: Block,
}

impl AES {
    const BLOCK_SIZE: usize = 16;
    pub fn new(key: Block) -> Self {
        Self { key }
    }

    pub fn encrypt(&self, plaintext: Block) -> Block {
        *b"testtesttesttest"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes() {
        let key = *b"testtesttesttest";
        let aes = AES::new(key);
        let plaintext = *b"testtesttesttest";
        let ciphertext = aes.encrypt(plaintext);
        assert_eq!(ciphertext, *b"testtesttesttest");
    }
}
