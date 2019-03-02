mod bit {
    use std::io::{Write, Seek, SeekFrom};
    use BitCursor;

    #[test]
    fn write_u8_to_bits() {
        let mut to: [bool; 52] = [
            false, true, true, false, true, false, true, false, true, true, true, true, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
        ];
        let from: [u8; 2] = [255, 255];
        let mut bcurs = BitCursor::new(&mut to[..]);
        bcurs.write(&from);
    }
}