mod u8 {
    use std::io::{Seek, SeekFrom, Write};
    use BitCursor;
    #[test]
    fn write() {
        let mut data: [u32; 3] = [
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&mut data[..]);
        let _ = bcurs.seek(SeekFrom::Start(6));
        let buf = [1];
        let _ = bcurs.write(&buf);
    }
}
