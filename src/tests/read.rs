mod u8 {
    use BitCursor;
    // test algo
//    #[test]
//    fn s() {
//        let first: u16 = 0b0010110000000001;
//        let last: u16 = 0b0100101111000001;
//        let list = [first, last];
//        let bpos: usize = 10;
//        let ref_size = 16;
//        let mut ret: u16 = 0;
//        println!("firs {:#018b}\nlast {:#018b}", first, last);
//        for (i, val) in list.iter().enumerate() {
//            let shifted = match bpos.checked_sub(ref_size * i) {
//                Some(sub) => {
//                    *val << sub
//                }
//                None => {
//                    *val >> ((bpos as i128) - ((ref_size * i) as i128)).wrapping_neg()
//                }
//            } as u16;
//            ret ^= shifted;
//            println!("\tval! {:#010b}", (ret >> 8) as u8);
//        }
//        //how much to shift over by? before cutting into U type (u8 u16 or w/e)
//        println!("{:b}", ret);
//    }
    #[test]
    fn read_u8_from_u8s() {
        let data: [u8; 3] = [0b01101010, 0b11110001, 0b01110100];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(10);
        let r = bcurs.read_unit::<u8>().unwrap();
        assert_eq!(0b11000101 as u8, r);
    }

    #[test]
    fn read_u8_from_u16s() {
        let data: [u16; 3] = [0b1000100001101010, 0b1001101011010001, 0b1000000101110100];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(10);
        let r = bcurs.read_unit::<u8>().unwrap();
        assert_eq!(0b10101010 as u8, r);
    }

    #[test]
    fn read_u8_from_u32s() {
        let data: [u32; 3] = [0b10010010001001011000100001101010, 0b10010010001001011000100001101010, 0b10010010001001011000100001101010];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(10);
        let r = bcurs.read_unit::<u8>().unwrap();
        assert_eq!(0b10010110 as u8, r);
    }

    #[test]
    fn read_u8_from_u64s() {
        let data: [u64; 3] = [0b1001001000100101100010000110101110010010001001011000100001101010, 0b1001001000100101100010000110101010010010001001011000100001101010, 0b1001001000100101100010000110101010010010001001011000100001101010];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(3*32+10);
        let r = bcurs.read_unit::<u8>().unwrap();
        assert_eq!(0b10010110 as u8, r);
    }
}
