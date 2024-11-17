pub struct Sha1 {}

impl Sha1 {
    const BLOCK_SIZE: usize = 512;
    const WORD_SIZE: usize = 32;
    const DIGEST_SIZE: usize = 160;

    fn pad(m: Vec<u8>) -> Vec<u8> {
        let l = m.len();
        let mods = (l + 1) % 64;
        let k = if mods > 56 {
            64 - (mods - 56)
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
}

#[cfg(test)]
mod tests {
    use super::Sha1;

    #[test]
    fn pad() {
        let tests = [vec![0xaa; 100]];

        for v in tests {
            println!("{:x?}", Sha1::pad(v));
        }
        panic!();
    }
}
