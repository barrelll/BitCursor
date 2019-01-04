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
        let data: [u8; 4] = [0b00011000, 0b10011001, 0b00011001, 0b10011011];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(15));
        let mut iter = 0;
        for (e, b) in bcurs.bytes().enumerate() {
            let _ = b.unwrap();
            iter = e;
        }
        assert_eq!(2, iter);
    }

    #[test]
    fn read_from_u8_to_chain() {
        let d: [u8; 4] = [0b00011000, 0b10011001, 0b00011001, 0b10011010];
        let d2: [u8; 4] = [0b00011000, 0b10011001, 0b00011001, 0b10011010];
        let bcurs = BitCursor::new(&d[..]);
        let handle = bcurs.chain(&d2[..]);
        let mut iter = 0;
        for (e, b) in handle.bytes().enumerate() {
            let _ = b.unwrap();
            iter = e;
        }
        assert_eq!(7, iter);
    }
}

mod u16 {
    use std::io::{Read, Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn read_from_u16s() {
        let data: [u16; 3] = [0b01101010, 0b11110001, 0b01110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let mut buf = vec![0, 0, 0, 0];
        let amt = bcurs.read(&mut buf).unwrap();
        assert_eq!(4, amt);
        assert_eq!(vec![0, 106, 0, 241], Vec::from(buf))
    }

    #[test]
    fn read_from_u16s_to_end() {
        let data: [u16; 3] = [0b01101010, 0b11110001, 0b01110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(13));
        let mut buf = vec![];
        let amt = bcurs.read_to_end(&mut buf).unwrap();
        assert_eq!(5, amt);
        assert_eq!(vec![64, 30, 32, 14, 128], Vec::from(buf))
    }

    #[test]
    fn read_from_u16s_to_string() {
        let data: [u16; 4] = [0b00011000, 0b10011001, 0b00011001, 0b10011010];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(1));
        let mut buf = String::new();
        let amt = bcurs.read_to_string(&mut buf).unwrap();
        assert_eq!(8, amt);
        assert_eq!(vec![0, 48, 1, 50, 0, 50, 1, 52], Vec::from(buf))
    }

    #[test]
    fn read_from_u16_to_bytes() {
        let data: [u16; 4] = [0b00011000, 0b10011001, 0b00011001, 0b10011011];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(15));
        let mut iter = 0;
        for (e, b) in bcurs.bytes().enumerate() {
            let _ = b.unwrap();
            iter = e;
        }
        assert_eq!(6, iter);
    }
}
