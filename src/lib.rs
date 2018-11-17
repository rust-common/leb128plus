extern crate num_traits;

use num_traits::sign::Unsigned;
use num_traits::int::PrimInt;
use num_traits::cast::AsPrimitive;

pub trait Write {
    fn write<V>(&mut self, v: V) -> std::io::Result<&mut Self>
        where V: Unsigned + PrimInt + AsPrimitive<u8>;

    fn write_u8(&mut self, v: u8) -> std::io::Result<&mut Self> { self.write(v) }

    fn write_u16(&mut self, v: u16) -> std::io::Result<&mut Self> { self.write(v) }

    fn write_u32(&mut self, v: u32) -> std::io::Result<&mut Self> { self.write(v) }

    fn write_u64(&mut self, v: u64) -> std::io::Result<&mut Self> { self.write(v) }

    fn write_u128(&mut self, v: u128) -> std::io::Result<&mut Self> { self.write(v) }
}

/// Write unsigned integer in the LEB128+ format to `std::io::Write` stream.
///
/// ## Examples
///
/// ```
/// let f = || -> std::io::Result<Vec<u8>> {
///     let mut v = vec![];
///     use leb128plus::Write;
///     std::io::Cursor::new(&mut v)
///         .write_u8(0)?
///         .write_u16(127)?
///         .write_u32(128)?
///         .write_u64(0xFF)?
///         .write_u128(0x17F)?
///         .write_u16(0x407F)?
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
        where V: Unsigned + PrimInt + AsPrimitive<u8>
    {
        loop {
            let x = v.as_();
            v = v >> 7;
            if v.is_zero() {
                self.write(&[x])?;
                break Ok(self);
            }
            self.write(&[0x80 | x])?;
            v = v - V::one();
        }
    }
}

pub trait Read {
    fn read<V>(&mut self) -> std::io::Result<V>
        where V: 'static + PrimInt + Unsigned, u8: AsPrimitive<V>;
    fn read_u8(&mut self) -> std::io::Result<u8> { self.read() }
    fn read_u16(&mut self) -> std::io::Result<u16> { self.read() }
    fn read_u32(&mut self) -> std::io::Result<u32> { self.read() }
    fn read_u64(&mut self) -> std::io::Result<u64> { self.read() }
    fn read_u128(&mut self) -> std::io::Result<u128> { self.read() }
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
/// assert_eq!(c.read_u8().unwrap(), 0);
/// assert_eq!(c.read_u16().unwrap(), 127);
/// assert_eq!(c.read_u32().unwrap(), 128);
/// assert_eq!(c.read_u64().unwrap(), 0xFF);
/// assert_eq!(c.read_u128().unwrap(), 0x17F);
/// assert_eq!(c.read_u16().unwrap(), 0x407F);
/// assert_eq!(c.read_u32().unwrap(), 0x4080);
/// assert!(match c.read_u64() {
///     Result::Err(_) => true,
///     _ => false
/// });
/// ```
impl<T: std::io::Read> Read for T {
    fn read<V>(&mut self) -> std::io::Result<V>
        where
            V: 'static + PrimInt + Unsigned,
            u8: AsPrimitive<V>
    {
        let mut result: V = V::zero();
        let mut shift = 0;
        loop {
            let mut v = [0];
            self.read_exact(&mut v)?;
            result = result + ((v[0].as_()) << shift);
            if v[0] < 128 {
                break Ok(result);
            }
            shift += 7;
        }
    }
}
