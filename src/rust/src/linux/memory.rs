use std::{fs::OpenOptions, os::unix::fs::FileExt};

use crate::{MemoryAccessor, StreamMem, SystemMem};

impl MemoryAccessor for StreamMem {

    fn read_buffer(&self, buf: &mut [u8], addr: usize) {
        self.mem.read_exact_at(buf, addr as u64).unwrap_or_default();
    }
    
    fn write_buffer(&self, buf: &[u8], addr: usize) {
        _ = self.mem.write_at(buf, addr as u64);
    }
}

impl StreamMem {
    pub fn new(pid: u32) -> Result<Self, std::io::Error> {
        Ok(Self {
            mem: OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("/proc/{pid}/mem"))?
        })
    }
}

impl SystemMem {
    pub fn new() -> Self {
        Self {}
    }
}