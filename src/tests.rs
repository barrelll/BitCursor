#![cfg(test)]
use BitCursor;

#[test]
fn not_byte_aligned_next_bits() {
    let bytes: &[u8] = &[255, 1, 1, 1, 1, 100, 32, 43, 65, 76, 88, 21, 1];
    let bc = BitCursor::new(bytes);
    bc.move_bit_cursor(4);
    let next_bits = bc.next_bits();
    assert_eq!(
        [15, 1, 1, 1, 1, 100, 32, 43, 65, 76, 88, 21, 1].to_vec(),
        next_bits.unwrap()
    );
}

#[test]
fn not_byte_aligned_next_bits_byte_aligned() {
    let bytes: &[u8] = &[255, 1, 1, 1, 1, 100, 32, 43, 65, 76, 88, 21, 1];
    let bc = BitCursor::new(bytes);
    bc.move_bit_cursor(4);
    let next_bits = bc.next_bits_byte_aligned();
    assert_eq!(
        [1, 1, 1, 1, 100, 32, 43, 65, 76, 88, 21, 1].to_vec(),
        next_bits.unwrap()
    );
}
