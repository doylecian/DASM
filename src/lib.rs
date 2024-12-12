pub mod core;
use core::decoder::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_a_byte_array() {
        let decoder = DummyDecoder;
        let decoded: Vec<char> = decoder.decode(&[97u8, 98u8, 99u8]).iter().map(char::to_ascii_lowercase).collect();
        println!("{:?}", decoded);
        assert_eq!(decoded, vec!['a', 'b', 'c']);
    }
}
