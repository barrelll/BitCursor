mod u8 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_u8() {
        let data: [u8; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(37)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(5, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_u8() {
        let data: [u8; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_cur_pos(5);
        let curs_pos = bcurs.seek(SeekFrom::Current(2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(5, curs_pos);
        assert_eq!(2, bit_pos);
    }

    #[test]
    fn seek_from_current_back_u8() {
        let data: [u8; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_cur_pos(5);
        let curs_pos = bcurs.seek(SeekFrom::Current(-15)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(3, curs_pos);
        assert_eq!(1, bit_pos);
    }

    #[test]
    fn seek_from_end_u8() {
        let data: [u8; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-3)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(5, bit_pos);
    }
}

mod u16 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_u16() {
        let data: [u16; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(52)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(3, curs_pos);
        assert_eq!(4, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_u16() {
        let data: [u16; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(8);
        bcurs.set_cur_pos(2);
        let curs_pos = bcurs.seek(SeekFrom::Current(2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(2, curs_pos);
        assert_eq!(10, bit_pos);
    }

    #[test]
    fn seek_from_current_back_u16() {
        let data: [u16; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(8);
        bcurs.set_cur_pos(2);
        let curs_pos = bcurs.seek(SeekFrom::Current(-15)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(1, curs_pos);
        assert_eq!(9, bit_pos);
    }

    #[test]
    fn seek_from_end_u16() {
        let data: [u16; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-12)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(4, bit_pos);
    }
}

mod u32 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_u32() {
        let data: [u32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(140)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(12, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_u32() {
        let data: [u32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(8);
        bcurs.set_cur_pos(1);
        let curs_pos = bcurs.seek(SeekFrom::Current(32)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(2, curs_pos);
        assert_eq!(8, bit_pos);
    }

    #[test]
    fn seek_from_current_back_u32() {
        let data: [u32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_cur_pos(1);
        let curs_pos = bcurs.seek(SeekFrom::Current(-2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(0, curs_pos);
        assert_eq!(30, bit_pos);
    }

    #[test]
    fn seek_from_end_u32() {
        let data: [u32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-120)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(1, curs_pos);
        assert_eq!(8, bit_pos);
    }
}

mod u64 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_u64() {
        let data: [u64; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(140)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(2, curs_pos);
        assert_eq!(12, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_u64() {
        let data: [u64; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(40);
        let curs_pos = bcurs.seek(SeekFrom::Current(32)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(1, curs_pos);
        assert_eq!(8, bit_pos);
    }

    #[test]
    fn seek_from_current_back_u64() {
        let data: [u64; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(32);
        let curs_pos = bcurs.seek(SeekFrom::Current(-2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(0, curs_pos);
        assert_eq!(30, bit_pos);
    }

    #[test]
    fn seek_from_end_u64() {
        let data: [u64; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-120)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(3, curs_pos);
        assert_eq!(8, bit_pos);
    }
}

mod u128 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_u128() {
        let data: [u128; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(140)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(1, curs_pos);
        assert_eq!(12, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_u128() {
        let data: [u128; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(40);
        let curs_pos = bcurs.seek(SeekFrom::Current(32)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(0, curs_pos);
        assert_eq!(72, bit_pos);
    }

    #[test]
    fn seek_from_current_back_u128() {
        let data: [u128; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(32);
        let curs_pos = bcurs.seek(SeekFrom::Current(-2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(0, curs_pos);
        assert_eq!(30, bit_pos);
    }

    #[test]
    fn seek_from_end_u128() {
        let data: [u128; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-120)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(8, bit_pos);
    }
}

mod i8 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_i8() {
        let data: [i8; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(37)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(5, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_i8() {
        let data: [i8; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_cur_pos(5);
        let curs_pos = bcurs.seek(SeekFrom::Current(2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(5, curs_pos);
        assert_eq!(2, bit_pos);
    }

    #[test]
    fn seek_from_current_back_i8() {
        let data: [i8; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_cur_pos(5);
        let curs_pos = bcurs.seek(SeekFrom::Current(-15)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(3, curs_pos);
        assert_eq!(1, bit_pos);
    }

    #[test]
    fn seek_from_end_i8() {
        let data: [i8; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-3)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(5, bit_pos);
    }
}

mod i16 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_i16() {
        let data: [i16; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(52)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(3, curs_pos);
        assert_eq!(4, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_i16() {
        let data: [i16; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(8);
        bcurs.set_cur_pos(2);
        let curs_pos = bcurs.seek(SeekFrom::Current(2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(2, curs_pos);
        assert_eq!(10, bit_pos);
    }

    #[test]
    fn seek_from_current_back_i16() {
        let data: [i16; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(8);
        bcurs.set_cur_pos(2);
        let curs_pos = bcurs.seek(SeekFrom::Current(-15)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(1, curs_pos);
        assert_eq!(9, bit_pos);
    }

    #[test]
    fn seek_from_end_i16() {
        let data: [i16; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-12)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(4, bit_pos);
    }
}

mod i32 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_i32() {
        let data: [i32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(140)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(12, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_i32() {
        let data: [i32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(8);
        bcurs.set_cur_pos(1);
        let curs_pos = bcurs.seek(SeekFrom::Current(32)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(2, curs_pos);
        assert_eq!(8, bit_pos);
    }

    #[test]
    fn seek_from_current_back_i32() {
        let data: [i32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_cur_pos(1);
        let curs_pos = bcurs.seek(SeekFrom::Current(-2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(0, curs_pos);
        assert_eq!(30, bit_pos);
    }

    #[test]
    fn seek_from_end_i32() {
        let data: [i32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-120)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(1, curs_pos);
        assert_eq!(8, bit_pos);
    }
}

mod i64 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_i64() {
        let data: [i64; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(140)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(2, curs_pos);
        assert_eq!(12, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_i64() {
        let data: [i64; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(40);
        let curs_pos = bcurs.seek(SeekFrom::Current(32)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(1, curs_pos);
        assert_eq!(8, bit_pos);
    }

    #[test]
    fn seek_from_current_back_i64() {
        let data: [i64; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(32);
        let curs_pos = bcurs.seek(SeekFrom::Current(-2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(0, curs_pos);
        assert_eq!(30, bit_pos);
    }

    #[test]
    fn seek_from_end_i64() {
        let data: [i64; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        println!("wat");
        let curs_pos = bcurs.seek(SeekFrom::End(-120)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(3, curs_pos);
        assert_eq!(8, bit_pos);
    }
}

mod i128 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn seek_from_start_i128() {
        let data: [i128; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::Start(140)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(1, curs_pos);
        assert_eq!(12, bit_pos);
    }

    #[test]
    fn seek_from_current_forward_i128() {
        let data: [i128; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(40);
        let curs_pos = bcurs.seek(SeekFrom::Current(32)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(0, curs_pos);
        assert_eq!(72, bit_pos);
    }

    #[test]
    fn seek_from_current_back_i128() {
        let data: [i128; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(32);
        let curs_pos = bcurs.seek(SeekFrom::Current(-2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(0, curs_pos);
        assert_eq!(30, bit_pos);
    }

    #[test]
    fn seek_from_end_i128() {
        let data: [i128; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-120)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
        assert_eq!(8, bit_pos);
    }
}
