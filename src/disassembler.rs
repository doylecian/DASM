use std::{ops::Range, sync::Arc};

use crate::{
    decoders::decoder::{Decoder, SharedDecoder},
    mem::{
        memory::{Memory, SharedMemory},
        memory_permission_manager::{MemoryAccessLevel, MemoryPermissionManager, SharedMemoryPermissionManager},
    },
    Bytes,
};

pub struct Disassembler<T> {
    pub decoder: SharedDecoder<T>,
    pub memory_reader: SharedMemory,
    pub permission_manager: SharedMemoryPermissionManager,
}

impl<T> Disassembler<T> {
    pub unsafe fn disassemble(&self, region: Range<usize>) -> Result<Vec<T>, String> {
        self.permission_manager
            .set_memory_access(&region, MemoryAccessLevel::READONLY)?;

        region
            .map(|address| {
                self.memory_reader
                    .read(address)
                    .map(|bytes| self.decoder.decode(bytes))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|instructions| instructions.into_iter().flatten().collect())
    }

    pub fn new(
        decoder: SharedDecoder<T>,
        memory_reader: SharedMemory,
        permission_manager: SharedMemoryPermissionManager,
    ) -> Self {
        Self {
            decoder,
            memory_reader,
            permission_manager,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use once_cell::sync::Lazy;
    use crate::{
        mem::{
            memory::DummyMemory,
            memory_permission_manager::{self, DummyPermissionManager},
        },
        X86Decoder,
        X86Instruction::{self, JMP},
    };
    use super::Disassembler;

    static DECODER: Lazy<Arc<X86Decoder>> = Lazy::new(|| Arc::new(*X86Decoder::new()));
    static MEMORY_READER: Lazy<Arc<DummyMemory>> = Lazy::new(|| Arc::new(*DummyMemory::new()));
    static MEMORY_PERMISSION_MANAGER: Lazy<Arc<DummyPermissionManager>> =
        Lazy::new(|| Arc::new(*DummyPermissionManager::new()));

    static disassembler: Lazy<Disassembler<X86Instruction>> = Lazy::new(|| {
        Disassembler::new(
            (*DECODER).clone(),
            (*MEMORY_READER).clone(),
            (*MEMORY_PERMISSION_MANAGER).clone(),
        )
    });

    #[test]
    fn x86_disassembler_memory_region_returns_instructions() {
        let result = unsafe { disassembler.disassemble(0x0..0x3) };
        assert_eq!(
            result.unwrap(),
            vec![
                X86Instruction::ADD,
                X86Instruction::ADD,
                X86Instruction::ADD
            ]
        );
    }

    #[test]
    fn disassembler_memory_region_access_denied() {
        let result = unsafe { disassembler.disassemble(0xDEADBEEF .. 0xDEADBEEF + 1) };
        assert_eq!(
            result,
            Err("ERROR_ACCESS_DENIED".to_owned())
        );
    }

}
