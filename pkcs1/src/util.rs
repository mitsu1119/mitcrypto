use rug::Integer;

fn os2ip(octets: Vec<u8>) -> Integer {
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

    use crate::util::os2ip;

    #[test]
    fn t_os2ip() {
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
            assert_eq!(os2ip(x), y);
        }
    }
}
