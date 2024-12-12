
use super::decoder::{Decoder, Instruction};

pub struct X86Decoder;

impl Decoder<X86Instruction> for X86Decoder {
    fn decode(&self, byte_array: &[u8]) -> Vec<X86Instruction> {
        byte_array.iter().map(|b| byte_to_instruction(b)).collect()
    }
}

fn byte_to_instruction(byte: &u8) -> X86Instruction {
    match byte {
        0x90 => X86Instruction::NOP,
        0xE9 => X86Instruction::JMP,
        _ => X86Instruction::ERR
    }
}


#[derive(Debug, PartialEq)]
pub enum X86Instruction {
    NOP = 0x90,
    JMP = 0xE9,
    ERR
}

impl Instruction for X86Instruction {}