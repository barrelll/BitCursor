#![allow(dead_code)]
mod tests;

use std::{
    cell::Cell,
    fmt::Debug,
    io::Cursor,
    ops::{Shl, Shr},
};

pub trait Bits: Shl + Shr + Sized + Debug + Clone {
    const SIZE: u8;
    fn bit_at(&self, idx: u8) -> Result<bool, std::io::ErrorKind>;
    fn shr(&self, by: u8) -> Self;
    fn shl(&self, by: u8) -> Self;
    fn cut(&self, by: u8) -> Self;
}

macro_rules! impl_bits {
    ($type: ty, $size: expr) => {
        impl Bits for $type {
            const SIZE: u8 = $size;
            fn bit_at(&self, idx: u8) -> Result<bool, std::io::ErrorKind> {
                if idx > Self::SIZE {
                    Err(std::io::ErrorKind::InvalidInput)
                } else {
                    Ok(*self & (1 << idx) > 0)
                }
            }
        
            fn shr(&self, by: u8) -> $type {
                *self >> by
            }
        
            fn shl(&self, by: u8) -> $type {
                *self << by
            }
        
            fn cut(&self, by: u8) -> $type {
                let x = *self << by;
                x >> by
            }
        }
    };
}

impl_bits!(u8, 8);
impl_bits!(u16, 16);
impl_bits!(u32, 32);
impl_bits!(u64, 64);
impl_bits!(u128, 128);

impl_bits!(i8, 8);
impl_bits!(i16, 16);
impl_bits!(i32, 32);
impl_bits!(i64, 64);
impl_bits!(i128, 128);

#[derive(Debug)]
pub struct BitCursor<'a, T: 'a + Bits> {
    idx: Cell<u8>,
    byte_cursor: Cursor<&'a [T]>,
}

impl<'a, T: 'a + Bits> BitCursor<'a, T> {
    fn bit_position(&self) -> u8 {
        self.idx.get()
    }

    fn byte_position(&self) -> u64 {
        self.byte_cursor.position()
    }

    fn move_bit_cursor(&self, idx: u8) {
        self.idx.set(idx);
    }

    fn move_byte_cursor(&mut self, idx: u64) {
        self.byte_cursor.set_position(idx);
        self.idx.set(0);
    }

    pub fn new(from: &'a [T]) -> BitCursor<'a, T> {
        let idx = Cell::new(0);
        let byte_cursor = Cursor::new(from);
        BitCursor { idx, byte_cursor }
    }

    pub fn is_byte_aligned(&self) -> bool {
        self.idx.get() % 8 == 7
    }

    pub fn next_bits(&self) -> Option<Vec<T>> {
        let byte_index = self.byte_position() as usize;
        let bytes = &self.byte_cursor.get_ref()[byte_index..];
        if !self.is_byte_aligned() || self.idx.get() != 0 {
            let mut v = bytes.to_vec();
            match v.first_mut() {
                Some(val) => *val = val.cut(self.idx.get()),
                None => return None,
            }
            Some(v)
        } else {
            Some(bytes.to_vec())
        }
    }

    pub fn next_bits_byte_aligned(&self) -> Option<Vec<T>> {
        let byte_index = self.byte_position() as usize;
        if !self.is_byte_aligned() || self.idx.get() != 0 {
            let bytes = &self.byte_cursor.get_ref()[byte_index + 1..];
            self.idx.set(0);
            let v = bytes.to_vec();
            Some(v)
        } else {
            let bytes = &self.byte_cursor.get_ref()[byte_index..];
            Some(bytes.to_vec())
        }
    }
}
