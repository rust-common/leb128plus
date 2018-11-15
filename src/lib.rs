pub fn from(get: &mut FnMut() -> u8) -> u64 {
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

pub fn to(mut v: u64, write: &mut FnMut(u8)) {
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

    fn test_leb128_plus(value: u64, values: &[u8]) {
        {
            let mut i = 0;
            to(
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
            let result = from(&mut || {
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
        test_leb128_plus(0x4080, &[0x80, 0x80, 0x00]);
    }
}
