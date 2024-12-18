use super::memory::Memory;
use crate::Bytes;
use std::{ptr, sync::Arc};

pub struct InternalMemory;

impl InternalMemory {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self {})
    }
}

// Used for  reading objects in the same virtual address space
impl Memory for InternalMemory {
    unsafe fn read(&self, address: usize, bytes_to_read: usize) -> Bytes {
        (*ptr::slice_from_raw_parts(address as *const u8, bytes_to_read)).to_vec()
    }

    unsafe fn write(&self, address: usize, data: Bytes) {
        (*ptr::slice_from_raw_parts_mut(address as *mut u8, data.len())).copy_from_slice(&data);
    }
}

#[cfg(test)]
mod test {
    use std::io::repeat;

    use crate::mem::{internal_memory::InternalMemory, memory::Memory};

    #[test]
    fn read_write_heap_data() {
        let heap_struct = Box::new([true, false, true, false, true, false]); // 010101 on heap
        let heap_ptr = unsafe { core::mem::transmute_copy::<_, usize>(&heap_struct) };

        assert_eq!(
            unsafe { InternalMemory.read(heap_ptr, 6) },
            [1, 0, 1, 0, 1, 0]
        );

        unsafe { InternalMemory.write(heap_ptr, vec![0, 0, 0, 0, 0, 0]) };

        assert_eq!(
            unsafe { InternalMemory.read(heap_ptr, 6) },
            [0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn read_heap_allocated_struct_reconstructable() {
        #[repr(packed(4))]
        #[derive(Debug, PartialEq)]
        struct HeapAllocatedStruct {
            some_4_byte_integer_field: i32,
            some_8_byte_integer_field: i64,
            some_boolean: bool,
        }

        let allocated_struct = HeapAllocatedStruct {
            some_4_byte_integer_field: 1234,
            some_8_byte_integer_field: 4321,
            some_boolean: true,
        };
        let heap_ptr = &allocated_struct as *const HeapAllocatedStruct;
        let bytes_read_from_heap = unsafe { InternalMemory.read(heap_ptr as usize, 16) };

        assert_eq!(&bytes_read_from_heap[0..4], 1234_i32.to_le_bytes());
        assert_eq!(&bytes_read_from_heap[4..12], 4321_i64.to_le_bytes());
        assert_eq!(bytes_read_from_heap[12], 1);

        let bytes_read_array: [u8; 16]  = bytes_read_from_heap.try_into().unwrap();
        let reconstructed_struct = unsafe { core::mem::transmute::<_, HeapAllocatedStruct>(bytes_read_array) };

        assert_eq!(allocated_struct, reconstructed_struct);
    }
}
