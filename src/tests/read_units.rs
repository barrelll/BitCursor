mod u8 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn read_u8_from_u8s() {
        let data: [u8; 3] = [0b01101010, 0b11110001, 0b01110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(10));
        let r = bcurs.read_unit::<u8>().unwrap();
        assert_eq!(0b11000101 as u8, r);
    }

    #[test]
    #[should_panic]
    fn read_u8_from_u8s_out_of_range() {
        let data: [u8; 3] = [0b01101010, 0b11110001, 0b01110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(17));
        let _ = bcurs.read_unit::<u8>().unwrap();
    }

    #[test]
    fn read_u8_from_u16s() {
        let data: [u16; 3] = [0b1000100001101010, 0b1001101011010001, 0b1000000101110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(10));
        let r = bcurs.read_unit::<u8>().unwrap();
        assert_eq!(0b10101010 as u8, r);
    }

    #[test]
    #[should_panic]
    fn read_u8_from_u16s_out_of_range() {
        let data: [u16; 3] = [0b1000100001101010, 0b1001101011010001, 0b1000000101110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(41));
        let _ = bcurs.read_unit::<u8>().unwrap();
    }

    #[test]
    fn read_u8_from_u32s() {
        let data: [u32; 3] = [
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(10));
        let r = bcurs.read_unit::<u8>().unwrap();
        assert_eq!(0b10010110 as u8, r);
    }

    #[test]
    #[should_panic]
    fn read_u8_from_u32s_out_of_range() {
        let data: [u32; 3] = [
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(89));
        let _ = bcurs.read_unit::<u8>().unwrap();
    }

    #[test]
    fn read_u8_from_u64s() {
        let data: [u64; 3] = [
            0b1001001000100101100010000110101110010010001001011000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(3 * 32 + 10));
        let r = bcurs.read_unit::<u8>().unwrap();
        assert_eq!(0b10010110 as u8, r);
    }

    #[test]
    #[should_panic]
    fn read_u8_from_u64s_out_of_range() {
        let data: [u64; 3] = [
            0b1001001000100101100010000110101110010010001001011000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(185));
        let _ = bcurs.read_unit::<u8>().unwrap();
    }

    #[test]
    fn read_u8_from_u128s() {
        let data: [u128; 3] = [
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101110010010001001011000100001101010,
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(3 * 32 + 25));
        let r = bcurs.read_unit::<u8>().unwrap();
        assert_eq!(0b11010101 as u8, r);
    }

    #[test]
    #[should_panic]
    fn read_u8_from_u128s_out_of_range() {
        let data: [u128; 3] = [
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101110010010001001011000100001101010,
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(((3 * 128) as i32 - 7) as u64));
        let _ = bcurs.read_unit::<u8>().unwrap();
    }
}

mod u16 {
    use std::io::{Seek, SeekFrom};
    use BitCursor;
    #[test]
    fn read_u16_from_u8s() {
        let data: [u8; 4] = [0b01101010, 0b11110001, 0b01110100, 0b10100001];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(10));
        let r = bcurs.read_unit::<u16>().unwrap();
        assert_eq!(0b1100010111010010 as u16, r);
    }

    #[test]
    #[should_panic]
    fn read_u16_from_u8s_out_of_range() {
        let data: [u8; 4] = [0b01101010, 0b11110001, 0b01110100, 0b10100001];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(17));
        let _ = bcurs.read_unit::<u16>().unwrap();
    }

    #[test]
    fn read_u16_from_u16s() {
        let data: [u16; 3] = [0b1000100001101010, 0b1001101011010001, 0b1000000101110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(10));
        let r = bcurs.read_unit::<u16>().unwrap();
        assert_eq!(0b1010101001101011 as u16, r);
    }

    #[test]
    #[should_panic]
    fn read_u16_from_u16s_out_of_range() {
        let data: [u16; 3] = [0b1000100001101010, 0b1001101011010001, 0b1000000101110100];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(33));
        let _ = bcurs.read_unit::<u16>().unwrap();
    }

    #[test]
    fn read_u16_from_u32s() {
        let data: [u32; 3] = [
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(31));
        let r = bcurs.read_unit::<u16>().unwrap();
        assert_eq!(0b0100100100010010 as u16, r);
    }

    #[test]
    #[should_panic]
    fn read_u16_from_u32s_out_of_range() {
        let data: [u32; 3] = [
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
            0b10010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(81));
        let _ = bcurs.read_unit::<u16>().unwrap();
    }

    #[test]
    fn read_u16_from_u64s() {
        let data: [u64; 3] = [
            0b1001001000100101100010000110101110010010001001011000100001101010,
            0b1001001000100101100010000110101010010010001001111000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(3 * 32 + 10));
        let r = bcurs.read_unit::<u16>().unwrap();
        assert_eq!(0b1001111000100001 as u16, r);
    }

    #[test]
    #[should_panic]
    fn read_u16_from_u64s_out_of_range() {
        let data: [u64; 3] = [
            0b1001001000100101100010000110101110010010001001011000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
            0b1001001000100101100010000110101010010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(178));
        let _ = bcurs.read_unit::<u16>().unwrap();
    }

    #[test]
    fn read_u16_from_u128s() {
        let data: [u128; 3] = [
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101110010010001001011000101101101010,
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(3 * 32 + 25));
        let r = bcurs.read_unit::<u16>().unwrap();
        assert_eq!(0b1101010100100100 as u16, r);
    }

    #[test]
    #[should_panic]
    fn read_u8_from_u128s_out_of_range() {
        let data: [u128; 3] = [
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101110010010001001011000100001101010,
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
        ];
        let mut bcurs = BitCursor::new(&data[..]);
        let _ = bcurs.seek(SeekFrom::Start(((3 * 128) as i32 - 15) as u64));
        let _ = bcurs.read_unit::<u16>().unwrap();
    }
}

//mod i8 {
//    use std::io::{Seek, SeekFrom};
//    use BitCursor;
//    #[test]
//    fn read_i8_from_i8s() {
//        let data: [u8; 3] = [0b01101010, 0b11110001, 0b01110100];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(10));
//        let r = bcurs.read_unit::<i8>().unwrap();
//        assert_eq!(0b11000101 as i8, r);
//    }
//
//    #[test]
//    #[should_panic]
//    fn read_i8_from_i8s_out_of_range() {
//        let data: [i8; 3] = [0b01101010, 0b11110001, 0b01110100];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(17));
//        let _ = bcurs.read_unit::<i8>().unwrap();
//    }
//
//    #[test]
//    fn read_i8_from_u16s() {
//        let data: [u16; 3] = [0b1000100001101010, 0b1001101011010001, 0b1000000101110100];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(10));
//        let r = bcurs.read_unit::<i8>().unwrap();
//        assert_eq!(0b10101010 as i8, r);
//    }
//
//    #[test]
//    #[should_panic]
//    fn read_i8_from_u16s_out_of_range() {
//        let data: [u16; 3] = [0b1000100001101010, 0b1001101011010001, 0b1000000101110100];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(41));
//        let _ = bcurs.read_unit::<i8>().unwrap();
//    }
//
//    #[test]
//    fn read_i8_from_u32s() {
//        let data: [u32; 3] = [
//            0b10010010001001011000100001101010,
//            0b10010010001001011000100001101010,
//            0b10010010001001011000100001101010,
//        ];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(10));
//        let r = bcurs.read_unit::<i8>().unwrap();
//        assert_eq!(0b10010110 as i8, r);
//    }
//
//    #[test]
//    #[should_panic]
//    fn read_i8_from_u32s_out_of_range() {
//        let data: [u32; 3] = [
//            0b10010010001001011000100001101010,
//            0b10010010001001011000100001101010,
//            0b10010010001001011000100001101010,
//        ];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(89));
//        let _ = bcurs.read_unit::<i8>().unwrap();
//    }
//
//    #[test]
//    fn read_i8_from_u64s() {
//        let data: [u64; 3] = [
//            0b1001001000100101100010000110101110010010001001011000100001101010,
//            0b1001001000100101100010000110101010010010001001011000100001101010,
//            0b1001001000100101100010000110101010010010001001011000100001101010,
//        ];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(3 * 32 + 10));
//        let r = bcurs.read_unit::<i8>().unwrap();
//        assert_eq!(0b10010110 as i8, r);
//    }
//
//    #[test]
//    #[should_panic]
//    fn read_i8_from_u64s_out_of_range() {
//        let data: [u64; 3] = [
//            0b1001001000100101100010000110101110010010001001011000100001101010,
//            0b1001001000100101100010000110101010010010001001011000100001101010,
//            0b1001001000100101100010000110101010010010001001011000100001101010,
//        ];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(185));
//        let _ = bcurs.read_unit::<i8>().unwrap();
//    }
//
//    #[test]
//    fn read_i8_from_u128s() {
//        let data: [u128; 3] = [
//            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101110010010001001011000100001101010,
//            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
//            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
//        ];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(3 * 32 + 25));
//        let r = bcurs.read_unit::<i8>().unwrap();
//        assert_eq!(0b1010101 as i8, r);
//    }
//
//    #[test]
//    #[should_panic]
//    fn read_i8_from_u128s_out_of_range() {
//        let data: [u128; 3] = [
//            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101110010010001001011000100001101010,
//            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
//            0b10010010001001011000100001101011100100100010010110001000011010101001001000100101100010000110101010010010001001011000100001101010,
//        ];
//        let mut bcurs = BitCursor::new(&data[..]);
//        let _ = bcurs.seek(SeekFrom::Start(((3 * 128) as i32 - 7) as u64));
//        let _ = bcurs.read_unit::<i8>().unwrap();
//    }
//}
