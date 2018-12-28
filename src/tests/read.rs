mod i8 {
    use BitCursor;
    #[test]
    fn read_u8() {
        let data: [u16; 5] = [24576, 7, 12, 3, 5];
        let mut bcurs = BitCursor::new(&data[..]);
        let r = bcurs.read_u8();
        println!("{:?}", r);
    }
}