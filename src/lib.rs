pub fn to_leb128<F>(mut v: u64, write: &mut F) 
where F: FnMut(u8) {
    loop {
        let x = v as u8;      
        v = v >> 7;
        if v == 0 {
            write(x);
            break;
        }
        write(0x80 | x);
    }
}

pub fn from_leb128_plus<F>(get: &mut F) -> u64 
where F: FnMut() -> u8 {
    let mut result = 0;
    let mut shift = 0;
    loop {
        let v = get();
        result += (v as u64) << shift;
        if v < 128 {
            return result;
        }
        shift += 7;
    }
}

pub fn to_leb128_plus<F>(mut v: u64, write: &mut F)
where F: FnMut(u8) {
    loop {
        let x = v as u8;      
        v = v >> 7;
        if v == 0 {
            write(x);
            break;
        }
        write(0x80 | x);
        v = v - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_leb128(value: u64, values: &[u8]) {
        let mut i = 0;
        to_leb128(
            value,
            &mut |q| {
                assert_eq!(q, values[i]);
                i = i + 1;
            }
        );
        assert_eq!(i, values.len()) 
    }

    #[test]
    fn leb128() {
        test_leb128(0, &[0]);
        test_leb128(127, &[127]);
        test_leb128(128, &[128, 1]);
        test_leb128(0xFF, &[0xFF, 1]);
    }

    fn test_leb128_plus(value: u64, values: &[u8]) {
        {
            let mut i = 0;
            to_leb128_plus(
                value,
                &mut |q| {
                    assert_eq!(q, values[i]);
                    i = i + 1;
                }
            );
            assert_eq!(i, values.len())
        }
        {
            let mut i = 0;
            let result = from_leb128_plus(&mut || {
                let q = values[i];
                i = i + 1;
                return q;
            });
            assert_eq!(result, value);
            assert_eq!(i, values.len());
        }
    }

    #[test]
    fn leb128_plus() {
        test_leb128_plus(0, &[0]);
        test_leb128_plus(127, &[127]);
        test_leb128_plus(128, &[128, 0]);
        test_leb128_plus(0xFF, &[0xFF, 0]);
        test_leb128_plus(0x17F, &[0xFF, 1]);
        test_leb128_plus(0x407F, &[0xFF, 0x7F]);
    }
}
