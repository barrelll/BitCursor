mod bit {
    use std::io::{Seek, SeekFrom, Write};
    use BitCursor;

    #[test]
    fn write_u8_to_bits() {
        let mut to = [
            false, true, true, false, true, false, true, false, true, true, true, true, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
        ];
        let from: [u8; 2] = [255, 0];
        let mut bcurs = BitCursor::new(&mut to[..]);
        let _ = bcurs.write(&from).unwrap();
        let equals = [
            true, true, true, true, true, true, true, true, false, false, false, false, false,
            false, false, false, false, true, true, true, false, true, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
        let _ = bcurs.seek(SeekFrom::Start(10));
        let _ = bcurs.write(&from[..]).unwrap();
        let equals = [
            true, true, true, true, true, true, true, true, false, false, true, true, true, true,
            true, true, true, true, false, false, false, false, false, false, false, false, false,
            false, true, false, true, true, true, false, true, false, false, false, false, false,
            false, true, false, true, true, true, false, true, false, false, false, false,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
    }
}

mod u32 {
    use std::io::{Seek, SeekFrom, Write};
    use BitCursor;

    #[test]
    fn write_u8_to_u32s() {
        let mut to: [u32; 3] = [
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
        ];
        let from: [u8; 2] = [255, 0];
        let mut bcurs = BitCursor::new(&mut to[..]);
        let _ = bcurs.write(&from).unwrap();
        let equals: [u32; 3] = [
            0b11111111000000001000100001101010,
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
        let _ = bcurs.seek(SeekFrom::Start(26));
        let _ = bcurs.write(&from[..]).unwrap();
    }
}
