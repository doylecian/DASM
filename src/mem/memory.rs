use std::{fmt::Debug, sync::Arc};

use crate::Bytes;

pub trait Memory {
    unsafe fn read(&self, address: usize) -> Result<Bytes, String>;
    unsafe fn write(&self, address: usize, data: Bytes) -> Result<String, String>;
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
    unsafe fn read(&self, address: usize) -> Result<Vec<u8>, String> {
        Ok(vec![0x1])
     }

    unsafe fn write(&self, address: usize, data: Vec<u8>) -> Result<String, String> {
        Ok(format!("Successfully wrote {:?} to {:2X}", data, address))
    }
}