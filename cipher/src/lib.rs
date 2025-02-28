pub mod error;
mod log;

use std::fmt::Display;

use error::CipherError;
type Result<T> = std::result::Result<T, error::CipherError>;

type Word = [u8; 4];
type State = [Word; 4]; // [column][row]

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block([u8; AES::BLOCK_SIZE]);

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..AES::BLOCK_SIZE {
            write!(f, "{:02x}", self.0[i])?;
        }
        Ok(())
    }
}

#[cfg(test)]
prog_log!(aes_log, crate::Block);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AES {
    key: Vec<u8>,
}

impl AES {
    const BLOCK_SIZE: usize = 16;

    const S_BOX: &'static [u8] = &[
        0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB,
        0x76, 0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4,
        0x72, 0xC0, 0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71,
        0xD8, 0x31, 0x15, 0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2,
        0xEB, 0x27, 0xB2, 0x75, 0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6,
        0xB3, 0x29, 0xE3, 0x2F, 0x84, 0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB,
        0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF, 0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45,
        0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8, 0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5,
        0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2, 0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44,
        0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73, 0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A,
        0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB, 0xE0, 0x32, 0x3A, 0x0A, 0x49,
        0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79, 0xE7, 0xC8, 0x37, 0x6D,
        0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08, 0xBA, 0x78, 0x25,
        0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A, 0x70, 0x3E,
        0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E, 0xE1,
        0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
        0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB,
        0x16,
    ];
    const INV_S_BOX: &'static [u8] = &[
        0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7,
        0xFB, 0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE,
        0xE9, 0xCB, 0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42,
        0xFA, 0xC3, 0x4E, 0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49,
        0x6D, 0x8B, 0xD1, 0x25, 0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C,
        0xCC, 0x5D, 0x65, 0xB6, 0x92, 0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15,
        0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84, 0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7,
        0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06, 0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02,
        0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B, 0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC,
        0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73, 0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD,
        0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E, 0x47, 0xF1, 0x1A, 0x71, 0x1D,
        0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B, 0xFC, 0x56, 0x3E, 0x4B,
        0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4, 0x1F, 0xDD, 0xA8,
        0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F, 0x60, 0x51,
        0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF, 0xA0,
        0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
        0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C,
        0x7D,
    ];

    pub fn new(key: &[u8]) -> Result<Self> {
        if ![16, 25, 32].contains(&key.len()) {
            return Err(CipherError::ValueError(format!(
                "Invalid AES key length ({} bits)",
                key.len() * 8
            )));
        }
        Ok(Self { key: key.to_vec() })
    }

    pub fn encrypt(&self, plaintext: Block) -> Block {
        let ws = self.key_expansion();
        let mut state = AES::block_to_state(plaintext);
        let nr = self.num_rounds();

        AES::add_round_key(&mut state, &ws[..4]);
        for i in 0..(nr - 1) {
            AES::sub_bytes(&mut state);
            AES::shift_rows(&mut state);
            AES::mix_columns(&mut state);
            AES::add_round_key(&mut state, &ws[4 * i + 4..4 * i + 8]);
        }
        AES::sub_bytes(&mut state);
        AES::shift_rows(&mut state);
        AES::add_round_key(&mut state, &ws[4 * nr..]);

        AES::state_to_block(state)
    }

    fn sub_bytes(state: &mut State) {
        for column in state.iter_mut() {
            for byte in column {
                *byte = AES::S_BOX[*byte as usize];
            }
        }
        AES::log_state(&state);
    }

    fn shift_rows(state: &mut State) {
        (state[0], state[1], state[2], state[3]) = (
            [state[0][0], state[1][1], state[2][2], state[3][3]],
            [state[1][0], state[2][1], state[3][2], state[0][3]],
            [state[2][0], state[3][1], state[0][2], state[1][3]],
            [state[3][0], state[0][1], state[1][2], state[2][3]],
        );
        AES::log_state(&state);
    }

    fn mix_columns(state: &mut State) {
        fn xtime(x: u8) -> u8 {
            if x & 0b10000000 != 0 {
                (x << 1) ^ 0b11011
            } else {
                x << 1
            }
        }

        for column in state.iter_mut() {
            let u = column[0] ^ column[1] ^ column[2] ^ column[3];
            let v = column[0];
            column[0] ^= u ^ xtime(column[0] ^ column[1]);
            column[1] ^= u ^ xtime(column[1] ^ column[2]);
            column[2] ^= u ^ xtime(column[2] ^ column[3]);
            column[3] ^= u ^ xtime(column[3] ^ v);
        }
        AES::log_state(&state);
    }

    fn add_round_key(state: &mut State, round_key: &[Word]) {
        for i in 0..4 {
            for j in 0..4 {
                state[i][j] ^= round_key[i][j];
            }
        }
        AES::log_state(&state);
    }

    /*
     * self.key から 4*(Nr + 1) ワードのラウンド鍵を生成
     */
    fn key_expansion(&self) -> Vec<Word> {
        fn rot_word(word: Word) -> Word {
            [word[1], word[2], word[3], word[0]]
        }
        fn sub_word(word: Word) -> Word {
            [
                AES::S_BOX[word[0] as usize],
                AES::S_BOX[word[1] as usize],
                AES::S_BOX[word[2] as usize],
                AES::S_BOX[word[3] as usize],
            ]
        }
        fn word_xor(a: Word, b: Word) -> Word {
            [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
        }

        let nr = self.num_rounds();
        let nk = self.key.len() / 4;
        let rcon: &[u8] = &[0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

        let mut key_schedule = vec![];
        for i in 0..(4 * (nr + 1)) {
            if i < nk {
                key_schedule.push([
                    self.key[4 * i],
                    self.key[4 * i + 1],
                    self.key[4 * i + 2],
                    self.key[4 * i + 3],
                ]);
            } else if i % nk == 0 {
                let mut tmp = word_xor(
                    key_schedule[i - nk],
                    sub_word(rot_word(key_schedule[i - 1])),
                );
                tmp[0] ^= rcon[i / nk - 1];
                key_schedule.push(tmp);
            } else if nk > 6 && i % nk == 4 {
                key_schedule.push(word_xor(
                    key_schedule[i - nk],
                    sub_word(key_schedule[i - 1]),
                ));
            } else {
                key_schedule.push(word_xor(key_schedule[i - nk], key_schedule[i - 1]));
            }
        }
        key_schedule
    }

    // ラウンド数
    fn num_rounds(&self) -> usize {
        match self.key.len() {
            16 => 10,
            24 => 12,
            32 => 14,
            _ => unreachable!(),
        }
    }

    /*
     * ブロック <-> ステート
     */
    #[inline]
    fn state_to_block(state: State) -> Block {
        let mut block = [0; 16];
        for i in 0..4 {
            for j in 0..4 {
                block[i * 4 + j] = state[i][j];
            }
        }
        Block(block)
    }
    #[inline]
    fn block_to_state(block: Block) -> State {
        let mut state = [[0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                state[i][j] = block.0[i * 4 + j];
            }
        }
        state
    }

    #[inline]
    fn log_state(state: &State) {
        #[cfg(test)]
        aes_log::push(AES::state_to_block(*state));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        aes_log::clear();
    }

    #[test]
    fn test_aes() {
        setup();
        let key = b"+~\x15\x16(\xae\xd2\xa6\xab\xf7\x15\x88\t\xcfO<";
        let aes = AES::new(key).unwrap();
        let plaintext = b"k\xc1\xbe\xe2.@\x9f\x96\xe9=~\x11s\x93\x17*";
        let ciphertext = aes.encrypt(Block(*plaintext));

        let log = aes_log::get();
        println!("{}", log.borrow().len());
        for block in log.borrow().iter() {
            println!("{}", block);
        }

        panic!("ugya");
    }

    #[test]
    fn aes_invalid_key_length() {
        let key = b"testtesttest";
        let aes = AES::new(key);

        assert_eq!(
            aes,
            Err(CipherError::ValueError(
                "Invalid AES key length (96 bits)".to_string()
            ))
        );
    }
}
