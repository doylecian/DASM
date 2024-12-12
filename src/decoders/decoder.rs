pub trait Decoder<T: Instruction> {
    fn decode(&self, byte_array: &[u8]) -> Vec<T>;
}

pub trait Instruction {}