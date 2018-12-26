mod tests;

use std::io::{Cursor, Error, ErrorKind, Result, Seek, SeekFrom};

macro_rules! impl_byte {
    ( $( $x:ty, $y:expr ),* ) => {
        $(
            impl Byte for $x {
                const SIZE: u8 = $y;
                type SelfType = $x;

                fn add(&mut self, other: Self::SelfType) {
                    *self += other;
                }
                fn sub(&mut self, other: Self::SelfType) {
                    *self -= other;
                }
                fn mul(&mut self, other: Self::SelfType) {
                    *self *= other;
                }
                fn div(&mut self, other: Self::SelfType) {
                    *self /= other;
                }
                fn mdl(&mut self, other: Self::SelfType) {
                    *self %= other;
                }
                fn shr(&mut self, other: Self::SelfType) {
                    *self >>= other;
                }
                fn shl(&mut self, other: Self::SelfType) {
                    *self <<= other;
                }

                fn add_ret(self, other: Self::SelfType) -> Option<Self::SelfType> {
                    self.checked_add(other)
                }
                fn sub_ret(self, other: Self::SelfType) -> Option<Self::SelfType> {
                    self.checked_sub(other)
                }
                fn mul_ret(self, other: Self::SelfType) -> Option<Self::SelfType> {
                    self.checked_mul(other)
                }
                fn div_ret(self, other: Self::SelfType) -> Option<Self::SelfType> {
                    self.checked_div(other)
                }
                fn shr_ret(self, other: u32) -> Option<Self::SelfType> {
                    self.checked_shr(other)
                }
                fn shl_ret(self, other: u32) -> Option<Self::SelfType> {
                    self.checked_shl(other)
                }
                fn mdl_ret(self, other: Self::SelfType) -> Self::SelfType {
                    self % other
                }
            }
        )*
    };
}

pub trait Byte {
    const SIZE: u8;
    type SelfType;

    fn add(&mut self, other: Self::SelfType);
    fn sub(&mut self, other: Self::SelfType);
    fn mul(&mut self, other: Self::SelfType);
    fn div(&mut self, other: Self::SelfType);
    fn mdl(&mut self, other: Self::SelfType);
    fn shr(&mut self, other: Self::SelfType);
    fn shl(&mut self, other: Self::SelfType);

    fn add_ret(self, other: Self::SelfType) -> Option<Self::SelfType>;
    fn sub_ret(self, other: Self::SelfType) -> Option<Self::SelfType>;
    fn mul_ret(self, other: Self::SelfType) -> Option<Self::SelfType>;
    fn div_ret(self, other: Self::SelfType) -> Option<Self::SelfType>;
    fn shr_ret(self, other: u32) -> Option<Self::SelfType>;
    fn shl_ret(self, other: u32) -> Option<Self::SelfType>;
    fn mdl_ret(self, other: Self::SelfType) -> Self::SelfType;
}

impl_byte!(
    u8, 8, u16, 16, u32, 32, u64, 64, u128, 128, i8, 8, i16, 16, i32, 32, i64, 64, i128, 128
);

pub trait ByteIterator {
    type Item;
    fn max_size() -> u8;

    fn blen(&self) -> usize;

    fn get_slice(&self, x: usize, y: usize) -> &[Self::Item];
}

impl<T: Byte> ByteIterator for Vec<T> {
    type Item = T;
    fn max_size() -> u8 {
        T::SIZE
    }

    fn blen(&self) -> usize {
        self.len()
    }

    fn get_slice(&self, x: usize, y: usize) -> &[Self::Item] {
        &self[x..y]
    }
}

impl<T: Byte> ByteIterator for &[T] {
    type Item = T;
    fn max_size() -> u8 {
        T::SIZE
    }

    fn blen(&self) -> usize {
        self.len()
    }

    fn get_slice(&self, x: usize, y: usize) -> &[Self::Item] {
        &self[x..y]
    }
}

#[derive(Debug, Clone)]
pub struct BitCursor<T: ByteIterator> {
    bit_pos: u8,
    cursor: Cursor<T>,
}

impl<T: ByteIterator> BitCursor<T> {
    pub fn new(data: T) -> BitCursor<T> {
        BitCursor {
            bit_pos: 0,
            cursor: Cursor::new(data),
        }
    }

    pub fn byte_aligned(&self) -> bool {
        self.bit_pos == (T::max_size() - 1)
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
        let max = T::max_size();
        self.bit_pos = new % max;
        self.set_cur_pos((new / max) as u64);
    }

    pub fn set_cur_pos(&mut self, new: u64) {
        self.cursor.set_position(new);
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let curpos = self.cur_position();
        let bitpos = self.bit_pos;
        let data = self.cursor.get_ref();
        let itsize = T::max_size() / 8;
        for _ in 0..itsize {

        }
        Ok(0)
    }
}

impl<T: ByteIterator> Seek for BitCursor<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        // size will always be a byte since we can only do this for Cursor with type &[u8]
        let (base_pos, offset) = match pos {
            SeekFrom::Start(v) => {
                self.bit_pos = (v % (T::max_size() as u64)) as u8;
                let seek_to = v / (T::max_size() as u64);
                self.set_cur_pos(seek_to);
                return Ok(seek_to);
            }
            SeekFrom::Current(v) => {
                self.bit_pos = (v % (T::max_size() as i64)) as u8;
                let seek_to = v / (T::max_size() as i64);
                (self.cur_position(), seek_to)
            }
            SeekFrom::End(v) => {
                self.bit_pos = (v % (T::max_size() as i64)) as u8;
                let seek_to = v / (T::max_size() as i64);
                (self.cursor.get_ref().blen() as u64, seek_to)
            }
        };
        let new_pos = if offset >= 0 {
            base_pos.checked_add(offset as u64)
        } else {
            base_pos.checked_add((offset.wrapping_neg()) as u64)
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
