use std::io::Read;
use std::io::Result;
use std::io::Write;

/// Read `u64` from the LEB128+ format.
///
/// Examples
///
/// ```
/// let mut c = std::io::Cursor::new(&[
///     0,
///     127,
///     128, 0,
///     0xFF, 0,
///     0xFF, 1,
///     0xFF, 0x7F,
///     0x80, 0x80, 0,
///     128,
/// ]);
/// assert_eq!(leb128plus::read(&mut c).unwrap(), 0);
/// assert_eq!(leb128plus::read(&mut c).unwrap(), 127);
/// assert_eq!(leb128plus::read(&mut c).unwrap(), 128);
/// assert_eq!(leb128plus::read(&mut c).unwrap(), 0xFF);
/// assert_eq!(leb128plus::read(&mut c).unwrap(), 0x17F);
/// assert_eq!(leb128plus::read(&mut c).unwrap(), 0x407F);
/// assert_eq!(leb128plus::read(&mut c).unwrap(), 0x4080);
/// assert!(match leb128plus::read(&mut c) {
///     Result::Err(_) => true,
///     _ => false
/// });
/// ```
pub fn read(r: &mut Read) -> Result<u64> {
    let mut result = 0;
    let mut shift = 0;
    loop {
        let mut v = [0];
        r.read_exact(&mut v)?;
        result += (v[0] as u64) << shift;
        if v[0] < 128 {
            break Ok(result);
        }
        shift += 7;
    }
}

/// Write `u64` in the LEB128+ format.
///
/// Examples
///
/// ```
/// let mut v = vec![];
/// {
///     let mut c = std::io::Cursor::new(&mut v);
///     leb128plus::write(&mut c, 0);
///     leb128plus::write(&mut c, 127);
///     leb128plus::write(&mut c, 128);
///     leb128plus::write(&mut c, 0xFF);
///     leb128plus::write(&mut c, 0x17F);
///     leb128plus::write(&mut c, 0x407F);
///     leb128plus::write(&mut c, 0x4080);
/// }
/// assert_eq!(v, [
///     0,
///     127,
///     128, 0,
///     0xFF, 0,
///     0xFF, 1,
///     0xFF, 0x7F,
///     0x80, 0x80, 0x00
/// ]);
/// ```
pub fn write(w: &mut Write, mut v: u64) -> Result<()> {
    loop {
        let x = v as u8;
        v = v >> 7;
        if v == 0 {
            w.write(&[x])?;
            break Ok(());
        }
        w.write(&[0x80 | x])?;
        v = v - 1;
    }
}
