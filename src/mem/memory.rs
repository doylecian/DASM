pub trait Memory {
    fn read(&self, address: usize) -> Result<Vec<u8>, &str>;
    fn write(&self, address: usize, data: usize) -> Result<(), &str>;
}

pub struct DummyMemory;

impl DummyMemory {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Memory for DummyMemory {
    fn read(&self, address: usize) -> Result<Vec<u8>, &str> {
       Ok(vec![0xE9])
    }

    fn write(&self, address: usize, data: usize) -> Result<(), &str> {
        Ok(())
    }
}