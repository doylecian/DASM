use std::sync::Arc;

use crate::Bytes;

pub trait Decoder<T> {
    fn decode(&self, byte_array: Bytes) -> Vec<T>;
}

pub type SharedDecoder<T> = Arc<dyn Decoder<T> + Send + Sync>;
pub type NonSharedDecoder<T> = Box<dyn Decoder<T>>;


pub trait Instruction {}