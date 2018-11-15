use std::io::Read;
use std::io::Result;
use std::io::Write;

/// Read `u64` from LEB128+ format.
///
/// Examples
///
/// ```
/// assert_eq!(leb128plus::read(&mut std::io::Cursor::new(&[0])).unwrap(), 0);
/// assert_eq!(leb128plus::read(&mut std::io::Cursor::new(&[127])).unwrap(), 127);
/// assert!(match leb128plus::read(&mut std::io::Cursor::new(&[128])) {
///     Result::Err(_) => true,
///     _ => false
/// });
/// assert_eq!(leb128plus::read(&mut std::io::Cursor::new(&[128, 0])).unwrap(), 128);
/// assert_eq!(leb128plus::read(&mut std::io::Cursor::new(&[0xFF, 0])).unwrap(), 0xFF);
/// assert_eq!(leb128plus::read(&mut std::io::Cursor::new(&[0xFF, 1])).unwrap(), 0x17F);
/// assert_eq!(leb128plus::read(&mut std::io::Cursor::new(&[0xFF, 0x7F])).unwrap(), 0x407F);
/// assert_eq!(leb128plus::read(&mut std::io::Cursor::new(&[0x80, 0x80, 0x00])).unwrap(), 0x4080);
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

/// Write `u64` in LEB128+ format.
///
/// Examples
///
/// ```
/// let mut v = vec![];
/// leb128plus::write(&mut std::io::Cursor::new(&mut v), 0);
/// assert_eq!(v, [0]);
/// ```
///
/// ```
/// let mut v = vec![];
/// leb128plus::write(&mut std::io::Cursor::new(&mut v), 127);
/// assert_eq!(v, [127]);
/// ```
///
/// ```
/// let mut v = vec![];
/// leb128plus::write(&mut std::io::Cursor::new(&mut v), 128);
/// assert_eq!(v, [128, 0]);
/// ```
///
/// ```
/// let mut v = vec![];
/// leb128plus::write(&mut std::io::Cursor::new(&mut v), 0xFF);
/// assert_eq!(v, [0xFF, 0]);
/// ```
///
/// ```
/// let mut v = vec![];
/// leb128plus::write(&mut std::io::Cursor::new(&mut v), 0x17F);
/// assert_eq!(v, [0xFF, 1]);
/// ```
///
/// ```
/// let mut v = vec![];
/// leb128plus::write(&mut std::io::Cursor::new(&mut v), 0x407F);
/// assert_eq!(v, [0xFF, 0x7F]);
/// ```
///
/// ```
/// let mut v = vec![];
/// leb128plus::write(&mut std::io::Cursor::new(&mut v), 0x4080);
/// assert_eq!(v, [0x80, 0x80, 0x00]);
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
