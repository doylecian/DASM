use std::{ops::Range, sync::Arc};

use crate::Bytes;

pub trait MemoryPermissionManager {
    unsafe fn set_memory_access(&self, memory_region: &Range<usize>, access_level: MemoryAccessLevel) -> Result<(), String>;  // TODO: Ensure correct handling of reads across page boundaries
}

pub type SharedMemoryPermissionManager = Arc<dyn MemoryPermissionManager + Send + Sync>;
pub type NonSharedMemoryPermissionManager = Box<dyn MemoryPermissionManager>;

#[derive(PartialEq)]
pub enum MemoryAccessLevel {
    READONLY,
    WRITEONLY,
    READWRITE
}

pub struct DummyPermissionManager;

impl DummyPermissionManager {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl MemoryPermissionManager for DummyPermissionManager {
    unsafe fn set_memory_access(&self, memory_region: &Range<usize>, access_level: MemoryAccessLevel) -> Result<(), String> {
        if memory_region.start % 2 == 0 && access_level == MemoryAccessLevel::READWRITE {
            Err("ERROR_ACCESS_DENIED".to_owned())
        } else {
            Ok(())
        }
     }
}
