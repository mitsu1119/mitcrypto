pub struct Sha1 {}

impl Sha1 {
    // bytes
    const BLOCK_SIZE: usize = 64;
    const WORD_SIZE: usize = 4;
    const DIGEST_SIZE: usize = 20;
    const IV: [[u8; Self::WORD_SIZE]; Self::DIGEST_SIZE / Self::WORD_SIZE] = [
        [0x67, 0x45, 0x23, 0x01],
        [0xef, 0xcd, 0xab, 0x89],
        [0x98, 0xba, 0xdc, 0xfe],
        [0x10, 0x32, 0x54, 0x76],
        [0xc3, 0xd2, 0xe1, 0xf0],
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
}

#[cfg(test)]
mod tests {
    use super::Sha1;

    #[test]
    fn pad() {
        let tests = [vec![0xaa; 100]];

        for v in tests {
            println!("{:x?}", Sha1::parse(Sha1::pad(v)));
        }
        panic!();
    }
}
