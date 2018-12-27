mod tests;

use std::convert::{From, Into};
use std::fmt::{Debug, Display};
use std::io::{Cursor, Error, ErrorKind, Result, Seek, SeekFrom};
use std::marker::PhantomData;
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
{
    const SIZE: u8;
    fn unitfrom(val: u128) -> Self;
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
pub struct BitCursor<'a, I: Unit, T: Into<UnitArr<'a, I>>> {
    bit_pos: u8,
    cursor: Cursor<UnitArr<'a, I>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, I: Unit, T: Into<UnitArr<'a, I>>> BitCursor<'a, I, T> {
    pub fn new(data: T) -> BitCursor<'a, I, T> {
        BitCursor {
            bit_pos: 0,
            cursor: Cursor::new(data.into()),
            _marker: PhantomData,
        }
    }

    pub fn byte_aligned(&self) -> bool {
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

    pub fn read_u8(&mut self) -> Result<u8> {
        //        let curpos = self.cur_position();
        //        let bitpos = self.bit_pos;
        let data = &self.cursor.get_ref().slice()[..];
        let itsize = UnitArr::<I>::unit_size() / 8;
        for mut val in &data[0..itsize as usize] {
            let something = *val * I::unitfrom(8);
            println!("{:?}", something);
        }
        Ok(0)
    }
}

impl<'a, I: Unit, T: Into<UnitArr<'a, I>>> Seek for BitCursor<'a, I, T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        // size will always be a byte since we can only do this for Cursor with type &[u8]
        let (base_pos, offset) = match pos {
            SeekFrom::Start(v) => {
                let unitsize = UnitArr::<I>::unit_size() as u64;
                self.bit_pos = (v % unitsize) as u8;
                let seek_to = v / unitsize;
                self.set_cur_pos(seek_to);
                return Ok(seek_to);
            }
            SeekFrom::Current(v) => {
                let unitsize = UnitArr::<I>::unit_size() as i128;
                let bits = self.bit_pos as i128 + ((self.cur_position() as i128) * unitsize);
                self.bit_pos = (((v as i128) + bits) % (unitsize)) as u8;
                let seek_to = ((v as i128) + bits) / unitsize;
                self.set_cur_pos(seek_to as u64);
                return Ok(seek_to as u64);
            }
            SeekFrom::End(v) => {
                let unitsize = UnitArr::<I>::unit_size() as i128;
                self.bit_pos = if v >= 0 {
                    ((v as i128) % unitsize) as u8
                } else {
                    ((v as i128) % unitsize).wrapping_neg() as u8
                };
                let seek_to = (v as i128) / unitsize;
                (self.cursor.get_ref().len() as u64, seek_to)
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
