pub trait Write {
    fn write(&mut self, v: u64) -> std::io::Result<&mut Self>;
}

/// Write `u64` in the LEB128+ format to `std::io::Write` stream.
///
/// ## Examples
///
/// ```
/// let mut f = || -> std::io::Result<()> {
///     let mut v = vec![];
///     use leb128plus::Write;
///     std::io::Cursor::new(&mut v)
///         .write(0)?
///         .write(127)?
///         .write(128)?
///         .write(0xFF)?
///         .write(0x17F)?
///         .write(0x407F)?
///         .write(0x4080)?;
///     assert_eq!(v, [
///         0,
///         127,
///         128, 0,
///         0xFF, 0,
///         0xFF, 1,
///         0xFF, 0x7F,
///         0x80, 0x80, 0x00
///     ]);
///     Ok(())
/// };
/// f().unwrap();
/// ```
impl<T: std::io::Write> Write for T {
    fn write(&mut self, mut v: u64) -> std::io::Result<&mut Self> {
        loop {
            let x = v as u8;
            v = v >> 7;
            if v == 0 {
                self.write(&[x])?;
                break Ok(self);
            }
            self.write(&[0x80 | x])?;
            v = v - 1;
        }
    }
}

pub trait Read {
    fn read(&mut self) -> std::io::Result<u64>;
}

/// Read `u64` in the LEB128+ format from `std::io::Read` stream.
///
/// ## Example
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
/// use leb128plus::Read;
/// assert_eq!(c.read().unwrap(), 0);
/// assert_eq!(c.read().unwrap(), 127);
/// assert_eq!(c.read().unwrap(), 128);
/// assert_eq!(c.read().unwrap(), 0xFF);
/// assert_eq!(c.read().unwrap(), 0x17F);
/// assert_eq!(c.read().unwrap(), 0x407F);
/// assert_eq!(c.read().unwrap(), 0x4080);
/// assert!(match c.read() {
///     Result::Err(_) => true,
///     _ => false
/// });
/// ```
impl<T: std::io::Read> Read for T {
    fn read(&mut self) -> std::io::Result<u64> {
        let mut result = 0;
        let mut shift = 0;
        loop {
            let mut v = [0];
            self.read_exact(&mut v)?;
            result += (v[0] as u64) << shift;
            if v[0] < 128 {
                break Ok(result);
            }
            shift += 7;
        }
    }
}
