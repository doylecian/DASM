use std::{fmt::Debug, ptr, sync::Arc};

use crate::Bytes;

pub trait Memory {
    unsafe fn read(&self, address: usize, bytes_to_read: usize) -> Bytes;
    unsafe fn write(&self, address: usize, data: Bytes);
}



pub type SharedMemory = Arc<dyn Memory + Send + Sync>;
pub type NonSharedMemory = Box<dyn Memory>;

pub struct DummyMemory;

impl DummyMemory {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl Memory for DummyMemory {
    unsafe fn read(&self, address: usize, bytes_to_read: usize) -> Bytes {
        vec![0x1, 0x2, 0x3]
     }

    unsafe fn write(&self, address: usize, data: Bytes) {
        format!("Successfully wrote {:?} to {:2X}", data, address);
    }
}


