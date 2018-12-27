#![cfg(test)]

mod seek {
    use ::BitCursor;
    use std::io::{Seek, SeekFrom};
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
        bcurs.set_bit_pos(40);
        let curs_pos = bcurs.seek(SeekFrom::Current(-15)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(0, curs_pos);
        assert_eq!(25, bit_pos);
    }

    #[test]
    fn seek_from_end_i32() {
        let data: [i32; 5] = [5, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let curs_pos = bcurs.seek(SeekFrom::End(-120)).unwrap();
        let bit_pos = bcurs.bit_position();
        assert_eq!(2, curs_pos);
        assert_eq!(24, bit_pos);
    }
}
