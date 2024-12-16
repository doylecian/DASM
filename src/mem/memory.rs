use crate::Bytes;

pub trait Memory {
    unsafe fn read(&self, address: usize) -> Result<Bytes, String>;
    unsafe fn write(&self, address: usize, data: Bytes) -> Result<String, String>;
}

pub struct DummyMemory;

impl DummyMemory {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
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