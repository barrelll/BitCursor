## BitCursor
[![Crates.io][crate-image]][link1] [![docs][docs-badge]][link2] [![MIT/Apache][license-badge]][link3]

[crate-image]: https://img.shields.io/badge/crates.io-1.0-important.svg
[link1]: https://crates.io/crates/bitcursor
[docs-badge]: https://img.shields.io/badge/docs-1.0-informational.svg
[link2]: https://docs.rs/bitcursor/0.1.0/bitcursor/
[license-badge]: https://img.shields.io/badge/license-MIT%2FApache-informational.svg
[link3]: LICENSE.md
____
Keeps track of the bit position for an in wrapped memory buffer, and provides it with a read, write, and seek implementation. Also provides some traits for reading any size primitive, unsigned or signed integer **ReadBits** && **ForceReadBits**

#### Examples
Read a u16 from a list of u8's, first from bit position 0 and then from bit position 2 + cursor position 1.

    use {BitCursor, Readbits};
    
    let data: [u8; 4] = [0b01101010, 0b11110001, 0b01110100, 0b10100001];
    let mut bcurs = BitCursor::new(&data[..]);
    let r = bcurs.read_bits::<u16>().unwrap();
    assert_eq!(0b0110101011110001 as u16, r);
    let _ = bcurs.seek(SeekFrom::Start(10));
    let r = bcurs.read_bits::<u16>().unwrap();
    assert_eq!(0b1100010111010010 as u16, r);


To see more examples see the tests module
