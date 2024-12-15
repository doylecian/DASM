use std::ops::Range;

use crate::{decoders::decoder::Decoder, mem::memory::Memory};

pub struct Disassembler<T> {
    pub decoder: Box<dyn Decoder<T>>,
    pub memory_reader: Box<dyn Memory>,
}

impl<T> Disassembler<T> {
    pub fn disassemble(&self, region: Range<usize>) -> Result<Vec<T>, &str> {
        region
            .map(|address| {
                self.memory_reader
                    .read(address)
                    .map(|bytes| self.decoder.decode(bytes))
            })
            .collect::<Result<Vec<_>, &str>>()
            .map(|instructions| instructions.into_iter().flatten().collect())
    }

    pub fn new(decoder: Box<dyn Decoder<T>>, memory_reader: Box<dyn Memory>) -> Self {
        Self {
            decoder,
            memory_reader,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{mem::memory::DummyMemory, X86Decoder, X86Instruction, X86Instruction::JMP};

    use super::*;

    #[test]
    fn disassemble_memory_region_x86() {
        let decoder = X86Decoder::new();
        let memory_reader = DummyMemory::new();
        let disassembler = Disassembler::new(decoder, memory_reader);

        let result = disassembler.disassemble(0x0..0x3);
        println!("{:?}", result.as_ref().unwrap());
        assert_eq!(result.unwrap(), vec![JMP, JMP, JMP]);
    }
}
