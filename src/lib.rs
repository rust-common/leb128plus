extern crate int;

use int::UInt;

pub trait WriteLeb128P: std::io::Write {
    /// Write the given unsigned integer in the LEB128+ format to `std::io::Write` stream.
    ///
    /// ## Examples
    ///
    /// ```
    /// let f = || -> std::io::Result<Vec<u8>> {
    ///     let mut v = vec![];
    ///     use leb128plus::WriteLeb128P;
    ///     {
    ///         let mut c = std::io::Cursor::new(&mut v);
    ///         c.write_leb128p(0_u8)?;
    ///         c.write_leb128p(127_u16)?;
    ///         c.write_leb128p(128_u32)?;
    ///         c.write_leb128p(0xFF_u64)?;
    ///         c.write_leb128p(0x17F_u128)?;
    ///         c.write_leb128p(0x407F_u16)?;
    ///         c.write_leb128p(0x4080_u32)?;
    ///     }
    ///     Ok(v)
    /// };
    /// assert_eq!(
    ///     f().unwrap(),
    ///     [
    ///         0,
    ///         127,
    ///         128, 0,
    ///         0xFF, 0,
    ///         0xFF, 1,
    ///         0xFF, 0x7F,
    ///         0x80, 0x80, 0x00
    ///     ]
    /// );
    /// ```
    fn write_leb128p<V: UInt>(&mut self, mut v: V) -> std::io::Result<()> {
        loop {
            let x = v.as_();
            v >>= 7;
            if v == V::_0 {
                self.write(&[x])?;
                break Ok(());
            }
            self.write(&[0x80 | x])?;
            v -= V::_1;
        }
    }
}

impl<T: std::io::Write> WriteLeb128P for T {}

pub trait ReadLeb128P: std::io::Read {
    /// Read an unsigned integer in the LEB128+ format from `std::io::Read` stream.
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
    /// use leb128plus::ReadLeb128P;
    /// assert_eq!(c.read_leb128p::<u8>().unwrap(), 0);
    /// assert_eq!(c.read_leb128p::<u16>().unwrap(), 127);
    /// assert_eq!(c.read_leb128p::<u32>().unwrap(), 128);
    /// assert_eq!(c.read_leb128p::<u64>().unwrap(), 0xFF);
    /// assert_eq!(c.read_leb128p::<u128>().unwrap(), 0x17F);
    /// assert_eq!(c.read_leb128p::<u16>().unwrap(), 0x407F);
    /// assert_eq!(c.read_leb128p::<u32>().unwrap(), 0x4080);
    /// assert!(match c.read_leb128p::<u64>() {
    ///     Result::Err(_) => true,
    ///     _ => false
    /// });
    /// ```
    fn read_leb128p<V: UInt>(&mut self) -> std::io::Result<V> {
        let mut result = V::_0;
        let mut shift = 0_u8;
        loop {
            let x = {
                let mut buf = [0];
                self.read_exact(&mut buf)?;
                buf[0]
            };
            result += V::from_u8(x) << shift;
            if x < 128 {
                break Ok(result);
            }
            shift += 7;
        }
    }
}

impl<T: std::io::Read> ReadLeb128P for T {}
