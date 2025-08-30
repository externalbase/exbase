use std::{ffi::c_void, fs::OpenOptions, os::unix::fs::FileExt};

use log::warn;

use crate::{bindings::{self, *}, error::Result, MemoryAccessor, StreamMem, SysMem, error::Error};

impl MemoryAccessor for StreamMem {
    fn read_buffer(&self, buf: &mut [u8], addr: usize) {
        if let Err(e) = self.mem.read_exact_at(buf, addr as u64) {
            warn!("libc read: {}", e)
        }
    }
    
    #[cfg(not(feature = "read_only"))]
    fn write_buffer(&self, buf: &[u8], addr: usize) {
        if let Err(e) = self.mem.write_at(buf, addr as u64) {
            warn!("libc write: {}", e)
        }
    }
}

impl MemoryAccessor for SysMem {
    fn read_buffer(&self, buf: &mut [u8], addr: usize) {
        let local_iov = iovec {
            iov_base: buf.as_mut_ptr() as *mut c_void,
            iov_len: buf.len()
        };
        let remote_iov = iovec {
            iov_base: addr as *mut c_void,
            iov_len: buf.len()
        };
        if process_vm_readv(self.pid, &local_iov, 1, &remote_iov, 1, 0) < 0 {
            let err = std::io::Error::last_os_error();
            warn!("process_vm_readv: {}", err);
        }
    }

    #[cfg(not(feature = "read_only"))]
    fn write_buffer(&self, buf: &[u8], addr: usize) {
        let local_iov = iovec {
            iov_base: buf.as_ptr() as *mut c_void,
            iov_len: buf.len(),
        };
        let remote_iov = iovec {
            iov_base: addr as *mut c_void,
            iov_len: buf.len(),
        };

        if process_vm_writev(self.pid, &local_iov, 1, &remote_iov, 1, 0) < 0 {

            let err = std::io::Error::last_os_error();
            warn!("process_vm_writev: {}", err);
        }
    }
}

impl StreamMem {
    pub fn new(pid: u32) -> Result<Self> {
        let mut mem_options = OpenOptions::new();
        let mem_options = mem_options.read(true);

        #[cfg(not(feature = "read_only"))]
        mem_options.write(true);
        #[cfg(feature = "read_only")]
        mem_options.write(false);

        Ok(Self {
            mem: mem_options.open(format!("/proc/{pid}/mem"))?
        })
    }
}

impl SysMem {
    pub fn new(pid: u32) -> Result<Self> {
        if !bindings::is_alive(pid as i32) {
            return Err(Error::other("Process is inactive"));
        }
        Ok(Self {
            pid: pid as i32
        })
    }
}