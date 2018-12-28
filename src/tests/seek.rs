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
        bcurs.set_bit_pos(40);
        let curs_pos = bcurs.seek(SeekFrom::Current(2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(5, curs_pos);
        assert_eq!(2, bit_pos);
    }

    #[test]
    fn seek_from_current_back_i8() {
        let data: [i8; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(40);
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
        bcurs.set_bit_pos(40);
        let curs_pos = bcurs.seek(SeekFrom::Current(2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(2, curs_pos);
        assert_eq!(10, bit_pos);
    }

    #[test]
    fn seek_from_current_back_i16() {
        let data: [i16; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(40);
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
        bcurs.set_bit_pos(40);
        let curs_pos = bcurs.seek(SeekFrom::Current(32)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(2, curs_pos);
        assert_eq!(8, bit_pos);
    }

    #[test]
    fn seek_from_current_back_i32() {
        let data: [i32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(160);
        let curs_pos = bcurs.seek(SeekFrom::Current(-2)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(4, curs_pos);
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
