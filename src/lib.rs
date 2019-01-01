mod tests;

use std::cmp::min;
use std::fmt::{Binary, Debug, Display};
use std::io::{BufRead, Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

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
    fn checked_shr(self, rhs: u32) -> Option<Self>;
    fn checked_shl(self, rhs: u32) -> Option<Self>;

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
                fn max_value() -> $x {
                    $x::max_value()
                }
                fn checked_shr(self, rhs: u32) -> Option<Self> {
                    self.checked_shr(rhs)
                }
                fn checked_shl(self, rhs: u32) ->  Option<Self> {
                    self.checked_shl(rhs)
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

trait SafeSlice<I> {
    fn slice(&self, x: usize, y: usize) -> Result<&[I]>;
}

impl<'a, I> SafeSlice<I> for &'a [I] {
    fn slice(&self, x: usize, y: usize) -> Result<&[I]> {
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
    fn force_slice(&self, x: usize, y: usize) -> &[I];
}

impl<'a, I> ForceSlice<I> for &'a [I] {
    fn force_slice(&self, x: usize, y: usize) -> &[I] {
        if y > self.len() {
            &self[x..]
        } else {
            &self[x..y]
        }
    }
}

impl<'a, I> ForceSlice<I> for &'a mut [I] {
    fn force_slice(&self, x: usize, y: usize) -> &[I] {
        if y > self.len() {
            &self[x..]
        } else {
            &self[x..y]
        }
    }
}

impl<'a, I> ForceSlice<I> for Vec<I> {
    fn force_slice(&self, x: usize, y: usize) -> &[I] {
        if y > self.len() {
            &self[x..]
        } else {
            &self[x..y]
        }
    }
}

impl<'a, I> ForceSlice<I> for &'a Vec<I> {
    fn force_slice(&self, x: usize, y: usize) -> &[I] {
        if y > self.len() {
            &self[x..]
        } else {
            &self[x..y]
        }
    }
}

impl<'a, I> ForceSlice<I> for &'a mut Vec<I> {
    fn force_slice(&self, x: usize, y: usize) -> &[I] {
        if y > self.len() {
            &self[x..]
        } else {
            &self[x..y]
        }
    }
}

#[derive(Debug, Clone)]
pub struct BitCursor<T> {
    bit_pos: u8,
    cursor: Cursor<T>,
}

impl<T> BitCursor<T> {
    pub fn new(data: T) -> BitCursor<T> {
        BitCursor {
            bit_pos: 0,
            cursor: Cursor::new(data),
        }
    }

    pub fn into_inner(self) -> T {
        self.cursor.into_inner()
    }

    pub fn get_ref(&self) -> &T {
        self.cursor.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.cursor.get_mut()
    }

    pub fn cur_position(&self) -> u64 {
        self.cursor.position()
    }

    pub fn bit_position(&self) -> u8 {
        self.bit_pos
    }

    pub fn set_bit_pos(&mut self, new: u8) {
        self.bit_pos = new;
    }

    pub fn set_cur_pos(&mut self, new: u64) {
        self.cursor.set_position(new);
    }
}

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
        let overlap = ((bpos + prc_size) / ref_size) as usize;
        if overlap > 0 && ((bpos + prc_size) % 8 != 0) || prc_size > ref_size {
            if ref_size >= prc_size {
                let mut ret = T::unitfrom(0);
                for (enumueration, val) in self
                    .get_ref()
                    .slice(cpos, cpos + overlap + 1)?
                    .iter()
                    .enumerate()
                {
                    let shifted = match bpos.checked_sub(ref_size * enumueration as u8) {
                        Some(sub) => *val << T::unitfrom(sub as u128),
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
                    Some(sub) => Ok(U::unitfrom((ret >> T::unitfrom(sub as u128)).into_u128())),
                    None => Ok(U::unitfrom(ret.into_u128())),
                }
            } else {
                let mut ret = U::unitfrom(0);
                for (enumueration, val) in self
                    .get_ref()
                    .slice(cpos, cpos + overlap + 1)?
                    .iter()
                    .enumerate()
                {
                    let val =
                        U::unitfrom(val.into_u128()) << U::unitfrom((prc_size - ref_size) as u128);
                    let shifted = match bpos.checked_sub(ref_size * enumueration as u8) {
                        Some(sub) => val << U::unitfrom(sub as u128),
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
                    Some(sub) => Ok(ret >> U::unitfrom(sub as u128)),
                    None => Ok(ret),
                }
            }
        } else {
            let ret = U::unitfrom(
                match self.get_ref().get(cpos) {
                    Some(v) => *v >> T::unitfrom((ref_size - prc_size - bpos) as u128),
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
        return Err(Error::new(ErrorKind::Other, "Not implemented yet"));
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
        let overlap = ((bpos + prc_size) / ref_size) as usize;
        if overlap > 0 && ((bpos + prc_size) % 8 != 0) || prc_size > ref_size {
            if ref_size >= prc_size {
                let mut ret = T::unitfrom(0);
                for (enumueration, val) in self
                    .get_ref()
                    .force_slice(cpos, cpos + overlap + 1)
                    .iter()
                    .enumerate()
                {
                    let shifted = match bpos.checked_sub(ref_size * enumueration as u8) {
                        Some(sub) => *val << T::unitfrom(sub as u128),
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
                    Some(sub) => Ok(U::unitfrom((ret >> T::unitfrom(sub as u128)).into_u128())),
                    None => Ok(U::unitfrom(ret.into_u128())),
                }
            } else {
                let mut ret = U::unitfrom(0);
                for (enumueration, val) in self
                    .get_ref()
                    .force_slice(cpos, cpos + overlap + 1)
                    .iter()
                    .enumerate()
                {
                    let val =
                        U::unitfrom(val.into_u128()) << U::unitfrom((prc_size - ref_size) as u128);
                    let shifted = match bpos.checked_sub(ref_size * enumueration as u8) {
                        Some(sub) => val << U::unitfrom(sub as u128),
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
                    Some(sub) => Ok(ret >> U::unitfrom(sub as u128)),
                    None => Ok(ret),
                }
            }
        } else {
            let ret = U::unitfrom(
                match self.get_ref().get(cpos) {
                    Some(v) => *v >> T::unitfrom((ref_size - prc_size - bpos) as u128),
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
        return Err(Error::new(ErrorKind::Other, "Not implemented yet"));
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
            println!("{:?}", i);
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
impl_read!(&'a [T], &'a mut [T], Vec<T>, &'a Vec<T>, &'a mut Vec<T>);

impl<'a, T: Unit> Read for BitCursor<T> {
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
                return Ok(enumeration)
            }
            let val = (T::unitfrom(*val as u128)
                << T::unitfrom((T::SIZE - (8 * enumeration) as u8) as u128))
                >> T::unitfrom(bpos as u128);
            inner[cpos + enumeration] |= val;
        }
        Ok(0)
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
