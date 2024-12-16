
use std::sync::Arc;

use crate::Bytes;

use super::decoder::{Decoder, Instruction};

pub struct X86Decoder;

impl Decoder<X86Instruction> for X86Decoder {
    fn decode(&self, byte_array: Bytes) -> Vec<X86Instruction> {
        byte_array.iter().map(|b| self.byte_to_instruction(*b)).collect()
    }
}

impl X86Decoder {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self {})
    }

    fn byte_to_instruction(&self, byte: u8) -> X86Instruction {
        _x86_byte_to_instruction(byte)
    }
    
}

impl Instruction for X86Instruction {}

include!(concat!(env!("OUT_DIR"), "/x86_generated.rs"));

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn code_gen_generates_function_with_enums() {
        assert_eq!(_x86_byte_to_instruction(0x0), X86Instruction::ADD);
        assert_eq!(_x86_byte_to_instruction(0x1), X86Instruction::ADD);
        assert_eq!(_x86_byte_to_instruction(0x2), X86Instruction::ADD);
        assert_eq!(_x86_byte_to_instruction(0xE9), X86Instruction::JMP);
        assert_eq!(_x86_byte_to_instruction(0x90), X86Instruction::NOP);
    }
}

