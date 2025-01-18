use std::str::FromStr;

use rug::{ops::Pow, Complete, Integer};

use crate::error::Pkcs1Error;

type Result<T> = std::result::Result<T, Pkcs1Error>;

pub fn i2osp(x: Integer, x_len: usize) -> Result<Vec<u8>> {
    if x < 0 {
        return Err(Pkcs1Error::ValueError(
            "integer must be non-negative".into(),
        ));
    }

    if x >= (Integer::ONE << (8 * x_len)).complete() {
        return Err(Pkcs1Error::ValueError("integer too large".into()));
    }

    let mut res = vec![0 as u8; x_len];
    let mut x = x;
    let mut cnt = 1;
    while x > 0 {
        res[x_len - cnt] = (x.clone() & Integer::from(0xff)).try_into().unwrap();
        x >>= 8;
        cnt += 1;
    }

    Ok(res)
}

pub fn os2ip(octets: Vec<u8>) -> Integer {
    let mut res = Integer::ZERO;

    if octets.len() > 0 {
        for i in &octets[..(octets.len() - 1)] {
            res += i;
            res <<= 8;
        }

        res += octets.last().unwrap();
    }
    res
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::{error::Pkcs1Error, util};

    #[test]
    fn i2osp() {
        let tests = [
            (Integer::from(-10), 1),
            (Integer::from(0x1234), 1),
            (Integer::from_str_radix("0", 16).unwrap(), 1),
            (Integer::from_str_radix("0", 16).unwrap(), 2),
            (Integer::from_str_radix("ab", 16).unwrap(), 1),
            (Integer::from(0x1234), 3),
            (
                Integer::from_str_radix("123456789abcdef123456789abcdef", 16).unwrap(),
                15,
            ),
            (
                Integer::from_str_radix("746573745f6461796f", 16).unwrap(),
                9,
            ),
        ];

        let res = [
            Err(Pkcs1Error::ValueError(
                "integer must be non-negative".into(),
            )),
            Err(Pkcs1Error::ValueError("integer too large".into())),
            Ok(b"\x00".to_vec()),
            Ok(b"\x00\x00".to_vec()),
            Ok(b"\xab".to_vec()),
            Ok(b"\x00\x12\x34".to_vec()),
            Ok(b"\x12\x34\x56\x78\x9a\xbc\xde\xf1\x23\x45\x67\x89\xab\xcd\xef".to_vec()),
            Ok(b"test_dayo".to_vec()),
        ];

        for ((x, x_len), y) in tests.into_iter().zip(res) {
            assert_eq!(util::i2osp(x, x_len), y);
        }
    }

    #[test]
    fn os2ip() {
        let tests = [
            b"\x00".to_vec(),
            b"\x00\x00".to_vec(),
            b"\xab".to_vec(),
            b"\x00\x12\x34".to_vec(),
            b"\x12\x34\x56\x78\x9a\xbc\xde\xf1\x23\x45\x67\x89\xab\xcd\xef".to_vec(),
            b"test_dayo".to_vec(),
        ];

        let res = [
            Integer::from_str_radix("0", 16).unwrap(),
            Integer::from_str_radix("0", 16).unwrap(),
            Integer::from_str_radix("ab", 16).unwrap(),
            Integer::from_str_radix("1234", 16).unwrap(),
            Integer::from_str_radix("123456789abcdef123456789abcdef", 16).unwrap(),
            Integer::from_str_radix("746573745f6461796f", 16).unwrap(),
        ];

        for (x, y) in tests.into_iter().zip(res) {
            assert_eq!(util::os2ip(x), y);
        }
    }
}
