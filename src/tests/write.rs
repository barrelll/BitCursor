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

mod u8 {
    use std::io::{Seek, SeekFrom, Write};
    use BitCursor;

    #[test]
    fn write_u8_to_u8s() {
        let mut to: [u8; 12] = [
            0b10010010, 0b00100101, 0b10001000, 0b01101010, 0b10010010, 0b00100101, 0b10001000,
            0b01101010, 0b10010010, 0b00100101, 0b10001000, 0b01101010,
        ];
        let from: [u8; 2] = [255, 0];
        let mut bcurs = BitCursor::new(&mut to[..]);
        let _ = bcurs.write(&from).unwrap();
        let equals: [u8; 12] = [
            0b11111111, 0b00000000, 0b10001000, 0b01101010, 0b10010010, 0b00100101, 0b10001000,
            0b01101010, 0b10010010, 0b00100101, 0b10001000, 0b01101010,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
        let _ = bcurs.seek(SeekFrom::Start(26));
        let _ = bcurs.write(&from[..]).unwrap();
        let equals: [u8; 12] = [
            0b11111111, 0b00000000, 0b10001000, 0b01111111, 0b11000000, 0b00100101, 0b10001000,
            0b01101010, 0b10010010, 0b00100101, 0b10001000, 0b01101010,
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
        let equals: [u32; 3] = [
            0b11111111000000001000100001111111,
            0b11000000001001011000100001101010,
            0b10010010001001011000100001101010,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
    }
}

mod u64 {
    use std::io::{Seek, SeekFrom, Write};
    use BitCursor;

    #[test]
    fn write_u8_to_u64s() {
        let mut to: [u64; 2] = [
            0b1001001000100101100010000110101010010010001001011000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
        ];
        let from: [u8; 2] = [255, 0];
        let mut bcurs = BitCursor::new(&mut to[..]);
        let _ = bcurs.write(&from).unwrap();
        let equals: [u64; 2] = [
            0b1111111100000000100010000110101010010010001001011000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
        let _ = bcurs.seek(SeekFrom::Start(26));
        let _ = bcurs.write(&from[..]).unwrap();
        let equals: [u64; 2] = [
            0b1111111100000000100010000111111111000000001001011000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
    }
}

mod u128 {
    use std::io::{Seek, SeekFrom, Write};
    use BitCursor;

    #[test]
    fn write_u8_to_u128s() {
        let mut to: [u128; 1] = [
            0b10010010001001011000100001101010100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
        ];
        let from: [u8; 2] = [255, 0];
        let mut bcurs = BitCursor::new(&mut to[..]);
        let _ = bcurs.write(&from).unwrap();
        let equals: [u128; 1] = [
            0b11111111000000001000100001101010100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
        let _ = bcurs.seek(SeekFrom::Start(26));
        let _ = bcurs.write(&from[..]).unwrap();
        let equals: [u128; 1] = [
            0b11111111000000001000100001111111110000000010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
        ];
        assert_eq!(&equals[..], &bcurs.get_ref()[..]);
    }
}
