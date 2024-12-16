use std::{any::Any, ops::Range, sync::Arc};

use crate::{
    decoders::{
        self,
        decoder::{Decoder, Instruction, SharedDecoder, SupportDecoders},
    },
    mem::{
        memory::{Memory, SharedMemory},
        memory_permission_manager::{
            MemoryAccessLevel, MemoryPermissionManager, SharedMemoryPermissionManager,
        },
    },
    Bytes, X86Decoder,
};

pub struct Disassembler<T> {
    pub decoder: SharedDecoder<T>,
    pub memory_reader: SharedMemory,
    pub permission_manager: SharedMemoryPermissionManager,
}

pub struct DisassemblerBuilder<T> {
    pub decoder: Option<SharedDecoder<T>>,
    pub memory_reader: Option<SharedMemory>,
    pub permission_manager: Option<SharedMemoryPermissionManager>,
}

impl<T> DisassemblerBuilder<T> {
    pub fn new() -> Self {
        DisassemblerBuilder {
            decoder: None,
            memory_reader: None,
            permission_manager: None,
        }
    }

    pub fn decoder(mut self, decoder: SharedDecoder<T>) -> Self {
        self.decoder = Some(decoder);
        self
    }

    pub fn memory_reader(mut self, memory_reader: SharedMemory) -> Self {
        self.memory_reader = Some(memory_reader);
        self
    }

    pub fn permission_manager(mut self, permission_manager: SharedMemoryPermissionManager) -> Self {
        self.permission_manager = Some(permission_manager);
        self
    }

    pub fn build(self) -> Option<Disassembler<T>> {
        self.decoder.and_then(|decoder| {
            self.memory_reader.and_then(|memory_reader| {
                self.permission_manager.and_then(|permission_manager| {
                    Some(Disassembler::new(
                        decoder,
                        memory_reader,
                        permission_manager,
                    ))
                })
            })
        })
    }
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
    use super::Disassembler;
    use crate::{
        disassembler::{self, DisassemblerBuilder},
        mem::{
            memory::DummyMemory,
            memory_permission_manager::{self, DummyPermissionManager},
        },
        X86Decoder,
        X86Instruction::{self, JMP},
    };
    use once_cell::sync::Lazy;
    use std::sync::Arc;

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
        let result = unsafe { disassembler.disassemble(0xDEADBEEF..0xDEADBEEF + 1) };
        assert_eq!(result, Err("ERROR_ACCESS_DENIED".to_owned()));
    }

    #[test]
    fn disassembler_builder_no_dependencies_provided() {
        let disassembler_build = DisassemblerBuilder::<X86Instruction>::new().build();
        assert!(disassembler_build.is_none());
    }

    #[test]
    fn disassembler_builder_all_dependencies_provided() {
        let disassembler_build = DisassemblerBuilder::<X86Instruction>::new()
            .decoder(X86Decoder::new_shared())
            .memory_reader(DummyMemory::new_shared())
            .permission_manager(DummyPermissionManager::new_shared())
            .build();

        let result = unsafe { disassembler.disassemble(0x0..0x3) };
        assert_eq!(
            result.unwrap(),
            vec![
                X86Instruction::ADD,
                X86Instruction::ADD,
                X86Instruction::ADD
            ]
        );
        
        assert!(disassembler_build.is_some());
    }
}
