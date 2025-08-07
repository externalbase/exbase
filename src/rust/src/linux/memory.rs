use std::fs::File;

use crate::Memory;

// process_vm_readv
pub struct SystemMem {

}

pub struct StreamMem {
    mem: File
}

impl Memory for SystemMem {
    fn read<T: Copy>(&self, address: *mut std::ffi::c_void) -> T {
        todo!()
    }

    fn read_buffer(&self, address: *mut std::ffi::c_void, len: usize) -> Vec<u8> {
        todo!()
    }
}

impl StreamMem {
    pub fn new(pid: u32) -> Result<Self, std::io::Error> {
        Ok(Self {
            mem: File::open(format!("/proc/{pid}/mem"))?,
        })
    }
}

impl SystemMem {
    pub fn new() -> Self {
        Self {}
    }
}