mod u8 {
    use std::io::{Read, Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn read_from_u8s() {
        let data: [u8; 3] = [0b01101010, 0b11110001, 0b01110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(13));
        let mut buf = vec![0, 0, 0, 0];
        let amt = bcurs.read(&mut buf).unwrap();
        assert_eq!(2, amt);
        assert_eq!(vec![46, 128, 0, 0], Vec::from(buf))
    }
}
