mod bit {
    use std::io::Write;
    use BitCursor;

    #[test]
    fn write_u8_to_bits() {
        let mut to: [bool; 52] = [
            false, true, true, false, true, false, true, false, true, true, true, true, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
            false, false, true, false, true, true, true, false, true, false, false, false, false,
        ];
        let from: [u8; 2] = [0b11111111 as u8, 0];
        let mut bcurs = BitCursor::new(&mut to[..]);
        println!("{:?}", bcurs);
        bcurs.write(&from).unwrap();
        println!("{:?}", bcurs);
    }
}
