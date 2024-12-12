pub trait Decoder<T> {
    fn decode(&self, byte_array: &[u8]) -> Vec<T>;
}


pub struct DummyDecoder;

impl Decoder<char> for DummyDecoder {
    fn decode(&self, byte_array: &[u8]) -> Vec<char> {
        byte_array.iter().map(|b| *b as char).collect()
    }
}

// impl DummyDecoder {
//     pub fn new() -> Self {
//         Self { }
//     }
// }
