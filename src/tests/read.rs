mod i8 {
    use BitCursor;
    #[test]
    fn read_u8() {
        let data: [u16; 3] = [0b0001000100101010, 0b1001101011010001, 0b101110100];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(8);
        let r = bcurs.read_8();
        println!("{:b} {:?}", r.unwrap(), bcurs.bit_position());
    }
}
