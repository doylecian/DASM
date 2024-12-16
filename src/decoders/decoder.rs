use crate::Bytes;

pub trait Decoder<T> {
    fn decode(&self, byte_array: Bytes) -> Vec<T>;
}

pub trait Instruction {}