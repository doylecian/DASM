pub trait Decoder<T> {
    fn decode(&self, byte_array: Vec<u8>) -> Vec<T>;
}

pub trait Instruction {}