mod u8 {
    use BitCursor;
    #[test]
    fn read_u8_from_u16s() {
        let data: [u16; 3] = [0b1000100001101010, 0b1001101011010001, 0b1000000101110100];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(10);
        let r = bcurs.read_unit::<u8>();
        println!("{:b} {:?}", r.unwrap(), bcurs.bit_position());
    }
}
