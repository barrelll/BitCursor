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

    #[test]
    fn read_from_u8s_to_end() {
        let data: [u8; 3] = [0b01101010, 0b11110001, 0b01110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(13));
        let mut buf = vec![];
        let amt = bcurs.read_to_end(&mut buf).unwrap();
        assert_eq!(2, amt);
        assert_eq!(vec![46, 128], Vec::from(buf))
    }

    #[test]
    fn read_from_u8s_to_string() {
        let data: [u8; 4] = [0b00011000, 0b10011001, 0b00011001, 0b10011010];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(1));
        let mut buf = String::new();
        let amt = bcurs.read_to_string(&mut buf).unwrap();
        assert_eq!(4, amt);
        assert_eq!(vec![49, 50, 51, 52], Vec::from(buf))
    }

    #[test]
    fn read_from_u8_to_bytes() {
        let data: [u8; 4] = [0b00011000, 0b10011001, 0b00011001, 0b10011010];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(15));
        for b in bcurs.bytes() {
            let _ = b.unwrap();
        }
    }
}
