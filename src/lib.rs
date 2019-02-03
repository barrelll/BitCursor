mod tests;

use std::cmp::min;
use std::fmt::{Debug, Display};
use std::io::{BufRead, Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

pub trait Unit:
    BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Sized
    + Copy
    + Clone
    + Debug
    + Display
{
    const SIZE: u8;
    fn unitfrom(val: u128) -> Self;
    fn shr(self, rhs: Self) -> Self;
    fn shl(self, rhs: Self) -> Self;
    fn checked_shr(self, rhs: u32) -> Option<Self>;
    fn checked_shl(self, rhs: u32) -> Option<Self>;
    fn read_bit_at(&self, rhs: u8) -> Option<bool>;

    fn into_u8(self) -> u8;
    fn into_u16(self) -> u16;
    fn into_u32(self) -> u32;
    fn into_u64(self) -> u64;
    fn into_u128(self) -> u128;

    fn into_i8(self) -> i8;
    fn into_i16(self) -> i16;
    fn into_i32(self) -> i32;
    fn into_i64(self) -> i64;
    fn into_i128(self) -> i128;
}

macro_rules! impl_unit {
    ( $( $x:ident, $y:expr ),* ) => {
        $(
            impl Unit for $x {
                const SIZE: u8 = $y;

                fn unitfrom(val: u128) -> $x {
                    val as $x
                }
                fn shr(self, rhs: Self) -> Self {
                    self >> rhs
                }
                fn shl(self, rhs: Self) -> Self {
                    self << rhs
                }
                fn checked_shr(self, rhs: u32) -> Option<Self> {
                    self.checked_shr(rhs)
                }
                fn checked_shl(self, rhs: u32) ->  Option<Self> {
                    self.checked_shl(rhs)
                }
                fn read_bit_at(&self, rhs: u8) -> Option<bool> {
                    let bitpos = 1 as $x;
                    if rhs >= Self::SIZE {
                        None
                    } else {
                        Some(*self & bitpos << rhs as $x > 1)
                    }
                }
                fn into_u8(self) -> u8 { self as u8 }
                fn into_u16(self) -> u16 { self as u16 }
                fn into_u32(self) -> u32 { self as u32 }
                fn into_u64(self) -> u64 { self as u64 }
                fn into_u128(self) -> u128 { self as u128 }

                fn into_i8(self) -> i8 { self as i8 }
                fn into_i16(self) -> i16 { self as i16 }
                fn into_i32(self) -> i32 { self as i32 }
                fn into_i64(self) -> i64 { self as i64 }
                fn into_i128(self) -> i128 { self as i128 }
            }
        )*
    };
}
impl_unit!(
    u8, 8, u16, 16, u32, 32, u64, 64, u128, 128, i8, 8, i16, 16, i32, 32, i64, 64, i128, 128
);

impl Unit for bool {
    const SIZE: u8 = 1;

    fn unitfrom(val: u128) -> bool {
        val > 0
    }
    fn shr(self, rhs: Self) -> Self {
        if rhs {
            false
        } else {
            self
        }
    }
    fn shl(self, rhs: Self) -> Self {
        if rhs {
            false
        } else {
            self
        }
    }
    fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs > 0 {
            None
        } else {
            Some(self)
        }
    }
    fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs > 0 {
            None
        } else {
            Some(self)
        }
    }
    fn read_bit_at(&self, rhs: u8) -> Option<bool> {
        if rhs > 0 {
            None
        } else {
            Some(*self)
        }
    }
    fn into_u8(self) -> u8 {
        self as u8
    }
    fn into_u16(self) -> u16 {
        self as u16
    }
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn into_u64(self) -> u64 {
        self as u64
    }
    fn into_u128(self) -> u128 {
        self as u128
    }

    fn into_i8(self) -> i8 {
        self as i8
    }
    fn into_i16(self) -> i16 {
        self as i16
    }
    fn into_i32(self) -> i32 {
        self as i32
    }
    fn into_i64(self) -> i64 {
        self as i64
    }
    fn into_i128(self) -> i128 {
        self as i128
    }
}

trait SafeSlice<I> {
    fn slice(&self, x: usize, y: usize) -> Result<&[I]>;
}

impl<'a, I> SafeSlice<I> for &'a [I] {
    fn slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Out of slice range! y {:?} should be less than {:?}",
                    y,
                    self.len()
                ),
            ))
        } else {
            Ok(&self[x..y])
        }
    }
}

impl<'a, I> SafeSlice<I> for &'a mut [I] {
    fn slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Out of slice range! y {:?} should be less than {:?}",
                    y,
                    self.len()
                ),
            ))
        } else {
            Ok(&self[x..y])
        }
    }
}

impl<'a, I> SafeSlice<I> for Vec<I> {
    fn slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Out of slice range! y {:?} should be less than {:?}",
                    y,
                    self.len()
                ),
            ))
        } else {
            Ok(&self[x..y])
        }
    }
}

impl<'a, I> SafeSlice<I> for &'a Vec<I> {
    fn slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Out of slice range! y {:?} should be less than {:?}",
                    y,
                    self.len()
                ),
            ))
        } else {
            Ok(&self[x..y])
        }
    }
}

impl<'a, I> SafeSlice<I> for &'a mut Vec<I> {
    fn slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Out of slice range! y {:?} should be less than {:?}",
                    y,
                    self.len()
                ),
            ))
        } else {
            Ok(&self[x..y])
        }
    }
}

trait ForceSlice<I> {
    fn force_slice(&self, x: usize, y: usize) -> Result<&[I]>;
}

impl<'a, I> ForceSlice<I> for &'a [I] {
    fn force_slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Ok(&self[x..])
        } else {
            Ok(&self[x..y])
        }
    }
}

impl<'a, I> ForceSlice<I> for &'a mut [I] {
    fn force_slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Ok(&self[x..])
        } else {
            Ok(&self[x..y])
        }
    }
}

impl<'a, I> ForceSlice<I> for Vec<I> {
    fn force_slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Ok(&self[x..])
        } else {
            Ok(&self[x..y])
        }
    }
}

impl<'a, I> ForceSlice<I> for &'a Vec<I> {
    fn force_slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Ok(&self[x..])
        } else {
            Ok(&self[x..y])
        }
    }
}

impl<'a, I> ForceSlice<I> for &'a mut Vec<I> {
    fn force_slice(&self, x: usize, y: usize) -> Result<&[I]> {
        if x > self.len() - 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("slice index starts at {}, but ends at {}", x, self.len()),
            ));
        }
        if y > self.len() {
            Ok(&self[x..])
        } else {
            Ok(&self[x..y])
        }
    }
}

/// A 'BitCursor' wraps an in memory buffer and provides it with a [`Seek`] implementation
/// provided that memory buffer impls the Unit trait
///
/// 'BitCursors' are used in memory buffers, any slice with types implementing Unit,
/// to allow it to read/write sizes of whatever unit implementation you pass in,
/// starting from the current bit position
///
/// It has implementations for some standard library traits such as
/// ['Seek']
/// ['Read']
/// ['Write']
/// ['BufRead']
///
/// # Examples
/// ```no_run
/// use std::io::{Read, Seek, SeekFrom};
/// use BitCursor;
/// fn read_from_bits() {
///     let data = [
///         false, true, true, false, true, false, true, false, true, true, true, true, false,
///         false, false, true, false, true, true, true, false, true, false, false,
///     ];
///     let mut bcurs = BitCursor::new(&data[..]);
///     let _ = bcurs.seek(SeekFrom::Start(13));
///     let mut buf = vec![0, 0, 0, 0];
///     let amt = bcurs.read(&mut buf).unwrap();
///     assert_eq!(2, amt);
///     assert_eq!(vec![46, 128, 0, 0], Vec::from(buf))
/// }
/// ```
#[derive(Debug, Clone)]
pub struct BitCursor<T> {
    bit_pos: u8,
    cursor: Cursor<T>,
}

impl<T> BitCursor<T> {
    /// Creates a new BitCursor wrapping the provided underlying in-memory buffer.
    ///
    /// Initial position for the bitcursor's unit cursor is 0, similarly the bit position will start at 0
    /// Similarly to std::io::Cursor writing to the Bitcursor starts with overwriting vector content, not appending it!
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use BitCursor;
    ///
    /// let buff = BitCursor::new(Vec::new());
    /// # fn force_inference(_: &BitCursor<Vec<u8>>) {}
    /// # force_inference(&buff);
    /// ```
    pub fn new(data: T) -> BitCursor<T> {
        BitCursor {
            bit_pos: 0,
            cursor: Cursor::new(data),
        }
    }

    /// Consumes the BitCursor, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use BitCursor;
    ///
    /// let buff = BitCursor::new(Vec::new());
    /// # fn force_inference(_: &BitCursor<Vec<u8>>) {}
    /// # force_inference(&buff);
    /// ```
    pub fn into_inner(self) -> T {
        self.cursor.into_inner()
    }

    /// Get a reference to the underlying value in this BitCursor
    ///
    /// # Examples
    ///
    /// ```
    /// use BitCursor;
    ///
    /// let buff = BitCursor::new(Vec::new());
    /// # fn force_inference(_: &BitCursor<Vec<u8>>) {}
    /// # force_inference(&buff);
    ///
    /// let reference = buff.get_ref();
    /// ```
    pub fn get_ref(&self) -> &T {
        self.cursor.get_ref()
    }

    /// Gets a mutable reference to the underlying value in this BitCursor.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying value as it may corrupt this cursor's position.
    ///
    /// # Examples
    ///
    /// ```
    /// use BitCursor;
    ///
    /// let mut buff = BitCursor::new(Vec::new());
    /// # fn force_inference(_: &BitCursor<Vec<u8>>) {}
    /// # force_inference(&buff);
    ///
    /// let reference = buff.get_mut();
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        self.cursor.get_mut()
    }

    /// Returns the current position of this BitCursor.
    ///
    /// # Examples
    ///
    /// ```
    /// use BitCursor;
    /// use std::io::prelude::*;
    /// use std::io::SeekFrom;
    ///
    /// let mut buff: BitCursor<Vec<u8>> = BitCursor::new(vec![1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(buff.cur_position(), 0);
    ///
    /// buff.seek(SeekFrom::Current(16)).unwrap();
    /// assert_eq!(buff.cur_position(), 2);
    ///
    /// buff.seek(SeekFrom::Current(-1)).unwrap();
    /// assert_eq!(buff.cur_position(), 1);
    /// ```
    pub fn cur_position(&self) -> u64 {
        self.cursor.position()
    }

    /// Returns the current bit position of this BitCursor.
    ///
    /// # Examples
    ///
    /// ```
    /// use BitCursor;
    /// use std::io::prelude::*;
    /// use std::io::SeekFrom;
    ///
    /// let mut buff: BitCursor<Vec<u8>> = BitCursor::new(vec![1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(buff.bit_position(), 0);
    ///
    /// buff.seek(SeekFrom::Current(16)).unwrap();
    /// assert_eq!(buff.bit_position(), 0);
    ///
    /// buff.seek(SeekFrom::Current(-1)).unwrap();
    /// assert_eq!(buff.bit_position(), 7);
    /// ```
    pub fn bit_position(&self) -> u8 {
        self.bit_pos
    }

    /// Sets the bit position of this BitCursor.
    /// does not wrap to the size of the Unit type
    /// also allows going over the length of the BitCursor
    /// it's recommended to use std::io::Seek
    ///
    /// # Examples
    ///
    /// ```
    /// use BitCursor;
    ///
    /// let mut buff: BitCursor<Vec<u8>> = BitCursor::new(vec![1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(buff.position(), 0);
    ///
    /// buff.set_bit_pos(2);
    /// assert_eq!(buff.bit_position(), 2);
    ///
    /// buff.set_bit_pos(4);
    /// assert_eq!(buff.bit_position(), 4);
    /// ```
    pub fn set_bit_pos(&mut self, new: u8) {
        self.bit_pos = new;
    }

    /// Sets the position of this BitCursor.
    /// also allows going over the length of the BitCursor
    /// it's recommended to use std::io::Seek
    ///
    /// # Examples
    ///
    /// ```
    /// use BitCursor;
    ///
    /// let mut buff: BitCursor<Vec<u8>> = BitCursor::new(vec![1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(buff.cur_position(), 0);
    ///
    /// buff.set_cur_pos(2);
    /// assert_eq!(buff.cur_position(), 2);
    ///
    /// buff.set_cur_pos(4);
    /// assert_eq!(buff.cur_position(), 4);
    /// ```
    pub fn set_cur_pos(&mut self, new: u64) {
        self.cursor.set_position(new);
    }
}

/// The 'ReadBits' trait allows reading bits of size unit (bool, u8, u32, etc.), at the given bit/cursor position
///
/// Implementors of 'ReadBits' are defined by one required method, ['read_bits'].
/// Each call to ['read_bits'] will attempt to read bits of unit size from the source,
/// given the source has enough bits for the unit size, otherwise it returns an Error value
///
/// Note: If the intended use is to grab each byte (size u8), Using an implementor of 'BufRead' will be more efficient.
///
/// # Examples
///
/// Read a u128 from a list of u8's first from bit position 0, and next and bit position 2, and cursor position 1 (8+2 = 10)
///
/// ```no_run
/// use {BitCursor, ReadBits};
/// use std::io::Seek;
/// use std::io::SeekFrom;
///
/// fn main() -> std::io::Result<()> {
///         let data: [u8; 22] = [
///            0b01101010, 0b11110001, 0b01110100, 0b10100001, 0b11100011, 0b11000000, 0b11110001,
///            0b01110100, 0b10100001, 0b11100011, 0b11000000, 0b01101010, 0b11110001, 0b01110100,
///            0b10100001, 0b11100011, 0b11000000, 0b11110001, 0b01110100, 0b10100001, 0b11100011,
///            0b11000000,
///         ];
///         let mut bcurs = BitCursor::new(&data[..]);
///         let r = bcurs.read_bits::<u128>().unwrap();
///         assert_eq!(
///            0b01101010111100010111010010100001111000111100000011110001011101001010000111100011110000000110101011110001011101001010000111100011 as u128,
///            r
///         );
///         let _ = bcurs.seek(SeekFrom::Start(10));
///         let r = bcurs.read_bits::<u128>().unwrap();
///         assert_eq!(
///            0b11000101110100101000011110001111000000111100010111010010100001111000111100000001101010111100010111010010100001111000111100000011 as u128,
///            r
///         );
///         Ok(())
/// }
/// ```
///
/// Read a u8 from a list of u32's
///
/// ```no_run
/// use {BitCursor, ReadBits};
/// use std::io::Seek;
/// use std::io::SeekFrom;
/// fn main() -> std::io::Result<()> {
///         let data: [u32; 3] = [
///             0b10010010001001011000100001101010,
///             0b10010010001001011000100001101010,
///             0b10010010001001011000100001101010,
///         ];
///         let mut bcurs = BitCursor::new(&data[..]);
///         let r = bcurs.read_bits::<u8>().unwrap();
///         assert_eq!(0b10010010 as u8, r);
///         let _ = bcurs.seek(SeekFrom::Start(10));
///         let r = bcurs.read_bits::<u8>().unwrap();
///         assert_eq!(0b10010110 as u8, r);
/// }
/// ```
pub trait ReadBits<T> {
    fn read_bits<U: Unit>(&mut self) -> Result<U>;
}

macro_rules! impl_readbits {
    ( $( $x:ty),* ) => {
        $(
impl<'a, T: Unit> ReadBits<T> for BitCursor<$x> {
    fn read_bits<U: Unit>(&mut self) -> Result<U> {
        let cpos = self.cur_position() as usize;
        let bpos = self.bit_position();
        let ref_size = T::SIZE;
        let prc_size = U::SIZE;
        let slice_add = if ref_size == 1 { 0 } else { 1 };
        let overlap = ((bpos + prc_size) / ref_size) as usize;
        if overlap > 0 && ((bpos + prc_size) % 8 != 0) || prc_size > ref_size {
            if ref_size >= prc_size {
                let mut ret = T::unitfrom(0);
                for (enumueration, val) in self
                    .get_ref()
                    .slice(cpos, cpos + overlap + slice_add)?
                    .iter()
                    .enumerate()
                {
                    let shifted = match bpos.checked_sub(ref_size * enumueration as u8) {
                        Some(sub) => val.shl(T::unitfrom(sub as u128)),
                        None => {
                            match val.checked_shr(
                                ((bpos as i128) - ((ref_size * enumueration as u8) as i128))
                                    .wrapping_neg() as u32,
                            ) {
                                Some(v) => v,
                                None => T::unitfrom(0),
                            }
                        }
                    };
                    ret |= shifted;
                }
                let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
                match ref_size.checked_sub(prc_size) {
                    Some(sub) => Ok(U::unitfrom((ret.shr(T::unitfrom(sub as u128))).into_u128())),
                    None => Ok(U::unitfrom(ret.into_u128())),
                }
            } else {
                let mut ret = U::unitfrom(0);
                for (enumueration, val) in self
                    .get_ref()
                    .slice(cpos, cpos + overlap + slice_add)?
                    .iter()
                    .enumerate()
                {
                    let val =
                        U::unitfrom(val.into_u128()).shl(U::unitfrom((prc_size - ref_size) as u128));
                    let shifted = match bpos.checked_sub(ref_size * enumueration as u8) {
                        Some(sub) => val.shl(U::unitfrom(sub as u128)),
                        None => {
                            match val.checked_shr(
                                ((bpos as i128) - ((ref_size * enumueration as u8) as i128))
                                    .wrapping_neg() as u32,
                            ) {
                                Some(v) => v,
                                None => U::unitfrom(0),
                            }
                        }
                    };
                    ret |= U::unitfrom(shifted.into_u128());
                }
                let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
                match ref_size.checked_sub(prc_size) {
                    Some(sub) => Ok(ret.shr(U::unitfrom(sub as u128))),
                    None => Ok(ret),
                }
            }
        } else {
            let ret = U::unitfrom(
                match self.get_ref().get(cpos) {
                    Some(v) => v.shr(T::unitfrom((ref_size - prc_size - bpos) as u128)),
                    None => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Cursor position outside of slice range",
                        ))
                    }
                }
                .into_u128(),
            );
            let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
            Ok(ret)
        }
    }
}
        )*
    };
}
impl_readbits!(&'a [T], &'a mut [T], Vec<T>, &'a Vec<T>, &'a mut Vec<T>);

impl<'a, T: Unit> ReadBits<T> for BitCursor<T> {
    fn read_bits<U: Unit>(&mut self) -> Result<U> {
        let cpos = self.cur_position();
        let bpos = self.bit_position();
        let ref_size = T::SIZE;
        let prc_size = U::SIZE;
        if cpos > 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Cursor position is out of range",
            ));
        } else {
            if prc_size + bpos > ref_size {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Not enough bits in type u{}", ref_size),
                ));
            } else {
                let ret = U::unitfrom(
                    self.get_ref()
                        .shr(T::unitfrom((ref_size - prc_size - bpos) as u128))
                        .into_u128(),
                );
                let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
                Ok(ret)
            }
        }
    }
}

pub trait ForceReadBits<T> {
    fn force_read_bits<U: Unit>(&mut self) -> Result<U>;
}

macro_rules! impl_forcereadbits {
    ( $( $x:ty),* ) => {
        $(
impl<'a, T: Unit> ForceReadBits<T> for BitCursor<$x> {
    fn force_read_bits<U: Unit>(&mut self) -> Result<U> {
        let cpos = self.cur_position() as usize;
        let bpos = self.bit_position();
        let ref_size = T::SIZE;
        let prc_size = U::SIZE;
        let slice_add = if ref_size == 1 { 0 } else { 1 };
        let overlap = ((bpos + prc_size) / ref_size) as usize;
        if overlap > 0 && ((bpos + prc_size) % 8 != 0) || prc_size > ref_size {
            if ref_size >= prc_size {
                let mut ret = T::unitfrom(0);
                for (enumueration, val) in self
                    .get_ref()
                    .force_slice(cpos, cpos + overlap + slice_add)?
                    .iter()
                    .enumerate()
                {
                    let shifted = match bpos.checked_sub(ref_size * enumueration as u8) {
                        Some(sub) => val.shl(T::unitfrom(sub as u128)),
                        None => {
                            match val.checked_shr(
                                ((bpos as i128) - ((ref_size * enumueration as u8) as i128))
                                    .wrapping_neg() as u32,
                            ) {
                                Some(v) => v,
                                None => T::unitfrom(0),
                            }
                        }
                    };
                    ret |= shifted;
                }
                let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
                match ref_size.checked_sub(prc_size) {
                    Some(sub) => Ok(U::unitfrom(ret.shr(T::unitfrom(sub as u128)).into_u128())),
                    None => Ok(U::unitfrom(ret.into_u128())),
                }
            } else {
                let mut ret = U::unitfrom(0);
                for (enumueration, val) in self
                    .get_ref()
                    .force_slice(cpos, cpos + overlap + slice_add)?
                    .iter()
                    .enumerate()
                {
                    let val =
                        U::unitfrom(val.into_u128()).shl(U::unitfrom((prc_size - ref_size) as u128));
                    let shifted = match bpos.checked_sub(ref_size * enumueration as u8) {
                        Some(sub) => val.shl(U::unitfrom(sub as u128)),
                        None => {
                            match val.checked_shr(
                                ((bpos as i128) - ((ref_size * enumueration as u8) as i128))
                                    .wrapping_neg() as u32,
                            ) {
                                Some(v) => v,
                                None => U::unitfrom(0),
                            }
                        }
                    };
                    ret |= U::unitfrom(shifted.into_u128());
                }
                let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
                match ref_size.checked_sub(prc_size) {
                    Some(sub) => Ok(ret.shr(U::unitfrom(sub as u128))),
                    None => Ok(ret),
                }
            }
        } else {
            let ret = U::unitfrom(
                match self.get_ref().get(cpos) {
                    Some(v) => v.shr(T::unitfrom((ref_size - prc_size - bpos) as u128)),
                    None => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Cursor position outside of slice range",
                        ))
                    }
                }
                .into_u128(),
            );
            let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
            Ok(ret)
        }
    }
}
        )*
    };
}
impl_forcereadbits!(&'a [T], &'a mut [T], Vec<T>, &'a Vec<T>, &'a mut Vec<T>);

impl<'a, T: Unit> ForceReadBits<T> for BitCursor<T> {
    fn force_read_bits<U: Unit>(&mut self) -> Result<U> {
        let cpos = self.cur_position();
        let bpos = self.bit_position();
        let ref_size = T::SIZE;
        let prc_size = U::SIZE;
        if cpos > 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Cursor position is out of range",
            ));
        } else {
            if prc_size + bpos >= ref_size {
                if ref_size >= prc_size {
                    let mut ret = T::unitfrom(0);
                    {
                        let val = self.get_ref();
                        let shifted = val.shl(T::unitfrom(bpos as u128));
                        ret |= shifted;
                    }
                    let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
                    match ref_size.checked_sub(prc_size) {
                        Some(sub) => Ok(U::unitfrom(ret.shr(T::unitfrom(sub as u128)).into_u128())),
                        None => Ok(U::unitfrom(ret.into_u128())),
                    }
                } else {
                    let mut ret = U::unitfrom(0);
                    {
                        let val = U::unitfrom(self.get_ref().into_u128())
                            .shl(U::unitfrom((prc_size - ref_size) as u128));
                        let shifted = val.shl(U::unitfrom(bpos as u128));
                        ret |= U::unitfrom(shifted.into_u128());
                    }
                    let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
                    match ref_size.checked_sub(prc_size) {
                        Some(sub) => Ok(ret.shr(U::unitfrom(sub as u128))),
                        None => Ok(ret),
                    }
                }
            } else {
                let ret = U::unitfrom(
                    self.get_ref()
                        .shr(T::unitfrom((ref_size - prc_size - bpos) as u128))
                        .into_u128(),
                );
                let _ = self.seek(SeekFrom::Current(prc_size as i64))?;
                Ok(ret)
            }
        }
    }
}

macro_rules! impl_seek {
    ( $( $x:ty),* ) => {
        $(
impl<'a, T: Unit> Seek for BitCursor<$x> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        // size will always be a byte since we can only do this for Cursor with type &[u8]
        let (base_pos, offset) = match pos {
            SeekFrom::Start(v) => {
                let unitsize = T::SIZE as u64;
                self.bit_pos = (v % unitsize) as u8;
                let seek_to = ((unitsize - self.bit_pos as u64) + v) / unitsize - 1;
                self.set_cur_pos(seek_to);
                return Ok(seek_to);
            }
            SeekFrom::Current(v) => {
                let unitsize = T::SIZE as i128;
                let bits = (self.bit_position() as i128)
                    + (self.cur_position() as i128 * unitsize)
                    + v as i128;
                self.bit_pos = (bits % unitsize) as u8;
                let seek_to = bits / unitsize;
                (self.cur_position(), seek_to - self.cur_position() as i128)
            }
            SeekFrom::End(v) => {
                let unitsize = T::SIZE as i128;
                let bits = (self.bit_position() as i128)
                    + (self.get_ref().len() as i128 * unitsize)
                    + v as i128;
                self.bit_pos = (bits % unitsize) as u8;
                let seek_to = bits / unitsize;
                (
                    self.get_ref().len() as u64,
                    seek_to - self.get_ref().len() as i128,
                )
            }
        };
        let new_pos = if offset >= 0 {
            base_pos.checked_add(offset as u64)
        } else {
            base_pos.checked_sub((offset.wrapping_neg()) as u64)
        };
        match new_pos {
            Some(n) => {
                self.set_cur_pos(n);
                Ok(n)
            }
            None => Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}
        )*
    };
}
impl_seek!(&'a [T], &'a mut [T], Vec<T>, &'a Vec<T>, &'a mut Vec<T>);

impl<'a, T: Unit> Seek for BitCursor<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        // size will always be a byte since we can only do this for Cursor with type &[u8]
        let (base_pos, offset) = match pos {
            SeekFrom::Start(v) => {
                let unitsize = T::SIZE as u64;
                self.bit_pos = (v % unitsize) as u8;
                let seek_to = ((unitsize - self.bit_pos as u64) + v) / unitsize - 1;
                self.set_cur_pos(seek_to);
                return Ok(seek_to);
            }
            SeekFrom::Current(v) => {
                let unitsize = T::SIZE as i128;
                let bits = (self.bit_position() as i128)
                    + (self.cur_position() as i128 * unitsize)
                    + v as i128;
                self.bit_pos = (bits % unitsize) as u8;
                let seek_to = bits / unitsize;
                (self.cur_position(), seek_to - self.cur_position() as i128)
            }
            SeekFrom::End(v) => {
                let unitsize = T::SIZE as i128;
                let bits = (self.bit_position() as i128) + (1 as i128 * unitsize) + v as i128;
                self.bit_pos = (bits % unitsize) as u8;
                let seek_to = bits / unitsize;
                (1 as u64, seek_to - 1 as i128)
            }
        };
        let new_pos = if offset >= 0 {
            base_pos.checked_add(offset as u64)
        } else {
            base_pos.checked_sub((offset.wrapping_neg()) as u64)
        };
        match new_pos {
            Some(n) => {
                self.set_cur_pos(n);
                Ok(n)
            }
            None => Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}

macro_rules! impl_read {
    ( $( $x:ty),* ) => {
        $(
impl<'a, T: Unit> Read for BitCursor<$x> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        for i in 0..buf.len() {
            buf[i] = match self.read_bits::<u8>() {
                Ok(val) => val,
                Err(_) => match self.force_read_bits::<u8>() {
                    Ok(val) => val,
                    Err(_) => return Ok(i),
                },
            }
        }
        Ok(buf.len())
    }
}
        )*
    };
}
impl_read!(&'a [T], &'a mut [T], Vec<T>, &'a Vec<T>, &'a mut Vec<T>, T);

macro_rules! impl_bufread {
    ( $( $x:ty),* ) => {
        $(
impl<'a> BufRead for BitCursor<$x>  {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        let amt = min(self.cur_position(), self.get_ref().len() as u64);
        Ok(&self.get_ref()[(amt as usize)..])
    }
    fn consume(&mut self, amt: usize) {
        self.set_cur_pos(amt as u64);
    }
}
        )*
    };
}
impl_bufread!(
    &'a [u8],
    &'a mut [u8],
    Vec<u8>,
    &'a Vec<u8>,
    &'a mut Vec<u8>
);

impl<'a, T: Unit> Write for BitCursor<&'a mut [T]> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let cpos = self.cur_position() as usize;
        let bpos = self.bit_position();
        let inner = self.get_mut();
        for (enumeration, val) in buf.iter().enumerate() {
            if cpos + enumeration > inner.len() {
                return Ok(enumeration);
            }
            let val = (T::unitfrom(*val as u128)
                .shl(T::unitfrom((T::SIZE - (8 * enumeration) as u8) as u128)))
            .shr(T::unitfrom(bpos as u128));
            inner[cpos + enumeration] |= val;
        }
        Ok(0)
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
