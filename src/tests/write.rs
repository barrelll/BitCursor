mod bit {
    use std::io::{Write, Seek, SeekFrom};
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
            true, true, true, true, true, true, true, true, false, false, true, true, true,
            true, true, true, true, true, false, false, false, false, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
    }
}

mod u32 {
//    use std::io::Write;
//    use BitCursor;

    #[test]
    fn write_u8_to_u32s() {
//        let mut to: [u32; 3] = [0b01101010, 0b11110001, 0b01110100];
//        let from: [u8; 2] = [255, 0];
    }
}
