mod i8 {
    use BitCursor;
    #[test]
    fn read_8() {
        let data: [u16; 3] = [0b1000100001101010, 0b1001101011010001, 0b1000000101110100];
        let mut bcurs = BitCursor::new(&data[..]);
        bcurs.set_bit_pos(10);
        let r = bcurs.read_8();
        println!("{:b} {:?}", r.unwrap(), bcurs.bit_position());
    }
}
