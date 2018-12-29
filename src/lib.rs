mod tests;

use std::convert::{From, Into};
use std::fmt::{Binary, Debug, Display};
use std::io::{Cursor, Error, ErrorKind, Result, Seek, SeekFrom};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

macro_rules! impl_byte {
    ( $( $x:ident, $y:expr ),* ) => {
        $(
            impl Unit for $x {
                const SIZE: u8 = $y;

                fn unitfrom(val: u128) -> $x {
                    val as $x
                }
                fn max_value() -> $x {
                    $x::max_value()
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

pub trait Unit:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Shl<Output = Self>
    + ShlAssign
    + Shr<Output = Self>
    + ShrAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Sized
    + Copy
    + Clone
    + Debug
    + Display
    + Binary
{
    const SIZE: u8;
    fn unitfrom(val: u128) -> Self;
    fn max_value() -> Self;

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

impl_byte!(
    u8, 8, u16, 16, u32, 32, u64, 64, u128, 128, i8, 8, i16, 16, i32, 32, i64, 64, i128, 128
);

#[derive(Debug, Clone, Copy)]
pub struct UnitArr<'a, T: Unit> {
    slice: &'a [T],
}

impl<'a, T: Unit> UnitArr<'a, T> {
    pub fn slice(&self) -> &[T] {
        self.slice
    }

    pub fn unit_size() -> u8 {
        T::SIZE
    }

    pub fn len(&self) -> usize {
        self.slice.len()
    }
}

impl<'a, T: Unit> From<&'a Vec<T>> for UnitArr<'a, T> {
    fn from(v: &'a Vec<T>) -> UnitArr<'a, T> {
        UnitArr { slice: &v[..] }
    }
}

impl<'a, T: Unit> From<&'a [T]> for UnitArr<'a, T> {
    fn from(s: &'a [T]) -> UnitArr<'a, T> {
        UnitArr { slice: s }
    }
}

#[derive(Debug, Clone)]
pub struct BitCursor<'a, I: Unit> {
    bit_pos: u8,
    cursor: Cursor<UnitArr<'a, I>>,
}

impl<'a, I: Unit> BitCursor<'a, I> {
    pub fn new<T: Into<UnitArr<'a, I>>>(data: T) -> BitCursor<'a, I> {
        BitCursor {
            bit_pos: 0,
            cursor: Cursor::new(data.into()),
        }
    }

    pub fn aligned(&self) -> bool {
        self.bit_pos == (UnitArr::<I>::unit_size() - 1)
    }

    pub fn into_inner(self) -> UnitArr<'a, I> {
        self.cursor.into_inner()
    }

    pub fn get_ref(&self) -> &UnitArr<'a, I> {
        self.cursor.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut UnitArr<'a, I> {
        self.cursor.get_mut()
    }

    pub fn cur_position(&self) -> u64 {
        self.cursor.position()
    }

    pub fn bit_position(&self) -> u8 {
        self.bit_pos
    }

    pub fn set_bit_pos(&mut self, new: u8) {
        let max = UnitArr::<I>::unit_size();
        self.bit_pos = new % max;
        self.set_cur_pos((new / max) as u64);
    }

    pub fn set_cur_pos(&mut self, new: u64) {
        self.cursor.set_position(new);
    }

    pub fn read_unit<U: Unit>(&mut self) -> Result<U> {
        let cpos = self.cur_position() as usize;
        let bpos = self.bit_position();
        let ref_size = I::SIZE;
        let prc_size = U::SIZE;
        let overlap = ((bpos + prc_size) / ref_size) as usize;
        if overlap > 0 && ((bpos + prc_size) % 8 != 0) {
            match self.get_ref().slice.get(cpos) {
                Some(first) => {
                    let mut val = *first
                        << I::unitfrom(
                            (match bpos.checked_sub(prc_size) {
                                Some(sub) => sub,
                                None => ((bpos as i32) - (prc_size as i32)).wrapping_neg() as u8,
                            }) as u128,
                        );
                    match self.get_ref().slice.get(cpos + 1 + overlap) {
                        Some(last) => {
                            let mut start_size = (ref_size - prc_size) as u128;
                            for v in &self.get_ref().slice[cpos + 1..cpos + overlap] {
                                val |= *v >> I::unitfrom(start_size);
                                start_size -= prc_size as u128;
                            }
                            println!("slice! {:?}", &self.get_ref().slice[cpos..cpos + overlap]);
                            val |= *last >> I::unitfrom((ref_size - (bpos - prc_size)) as u128);
                            let _ = self.seek(SeekFrom::Current(prc_size as i64));
                            Ok(U::unitfrom(val.into_u128()))
                        }
                        None => return Err(Error::new(ErrorKind::InvalidData, "Not enough data")),
                    }
                }
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Cursor position outside of slice range",
                    ))
                }
            }
        } else {
            match self.get_ref().slice.get(cpos) {
                Some(v) => {
                    let val = *v >> I::unitfrom((ref_size - prc_size - bpos) as u128);
                    let _ = self.seek(SeekFrom::Current(prc_size as i64));
                    Ok(U::unitfrom(val.into_u128()))
                }
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Cursor position outside of slice range",
                    ))
                }
            }
        }
    }
}

impl<'a, I: Unit> Seek for BitCursor<'a, I> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        // size will always be a byte since we can only do this for Cursor with type &[u8]
        let (base_pos, offset) = match pos {
            SeekFrom::Start(v) => {
                let unitsize = UnitArr::<I>::unit_size() as u64;
                self.bit_pos = (v % unitsize) as u8;
                let seek_to = ((unitsize - self.bit_pos as u64) + v) / unitsize - 1;
                self.set_cur_pos(seek_to);
                return Ok(seek_to);
            }
            SeekFrom::Current(v) => {
                let unitsize = UnitArr::<I>::unit_size() as i128;
                let bits = (self.bit_position() as i128)
                    + (self.cur_position() as i128 * unitsize)
                    + v as i128;
                self.bit_pos = (bits % unitsize) as u8;
                let seek_to = bits / unitsize;
                (self.cur_position(), seek_to - self.cur_position() as i128)
            }
            SeekFrom::End(v) => {
                let unitsize = UnitArr::<I>::unit_size() as i128;
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
