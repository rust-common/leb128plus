extern crate int;
extern crate num_traits;

use int::UInt;
use num_traits::cast::AsPrimitive;

pub trait Write {
    fn write<V>(&mut self, v: V) -> std::io::Result<&mut Self>
        where V: UInt, u8: AsPrimitive<V>;
}

/// Write the given unsigned integer in the LEB128+ format to `std::io::Write` stream.
///
/// ## Examples
///
/// ```
/// let f = || -> std::io::Result<Vec<u8>> {
///     let mut v = vec![];
///     use leb128plus::Write;
///     std::io::Cursor::new(&mut v)
///         .write(0_u8)?
///         .write(127_u16)?
///         .write(128_u32)?
///         .write(0xFF_u64)?
///         .write(0x17F_u128)?
///         .write(0x407F_u16)?
///         .write(0x4080_u32)?;
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
impl<T: std::io::Write> Write for T {
    fn write<V>(&mut self, mut v: V) -> std::io::Result<&mut Self>
        where V: UInt, u8: num_traits::cast::AsPrimitive<V>
    {
        loop {
            let x = v.as_();
            v >>= 7;
            if v == V::_0 {
                self.write(&[x])?;
                break Ok(self);
            }
            self.write(&[0x80 | x])?;
            v -= V::_1;
        }
    }
}

pub trait Read {
    fn read<V>(&mut self) -> std::io::Result<V>
        where V: UInt, u8: num_traits::cast::AsPrimitive<V>;
}

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
/// use leb128plus::Read;
/// assert_eq!(c.read::<u8>().unwrap(), 0);
/// assert_eq!(c.read::<u16>().unwrap(), 127);
/// assert_eq!(c.read::<u32>().unwrap(), 128);
/// assert_eq!(c.read::<u64>().unwrap(), 0xFF);
/// assert_eq!(c.read::<u128>().unwrap(), 0x17F);
/// assert_eq!(c.read::<u16>().unwrap(), 0x407F);
/// assert_eq!(c.read::<u32>().unwrap(), 0x4080);
/// assert!(match c.read::<u64>() {
///     Result::Err(_) => true,
///     _ => false
/// });
/// ```
impl<T: std::io::Read> Read for T {
    fn read<V>(&mut self) -> std::io::Result<V>
        where V: UInt, u8: num_traits::cast::AsPrimitive<V>
    {
        let mut result = V::_0;
        let mut shift = 0;
        loop {
            let x = {
                let mut buf = [0];
                self.read_exact(&mut buf)?;
                buf[0]
            };
            result += x.as_() << shift;
            if x < 128 {
                break Ok(result);
            }
            shift += 7;
        }
    }
}
