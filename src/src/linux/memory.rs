use std::{ffi::{c_int, c_ulong, c_void}, fs::OpenOptions, os::unix::fs::FileExt};

use crate::{MemoryAccessor, StreamMem, SysMem};

#[allow(non_camel_case_types)]
#[repr(C)]
struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize, // size_t
}

impl MemoryAccessor for StreamMem {
    fn read_buffer(&self, buf: &mut [u8], addr: usize) {
        self.mem.read_exact_at(buf, addr as u64).unwrap_or_default();
    }
    
    #[cfg(not(feature = "read_only"))]
    fn write_buffer(&self, buf: &[u8], addr: usize) {
        _ = self.mem.write_at(buf, addr as u64);
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
            eprintln!("process_vm_readv: {}", err);
        }
    }

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
            println!("process_vm_writev: {}", err);
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
        // is alive
        Ok(Self {
            pid: pid as i32
        })
    }
}

unsafe extern "C" {
    safe fn process_vm_readv(
        pid: c_int,
        local_iov: *const iovec, liovcnt: c_ulong,
        remote_iov: *const iovec, riovcnt: c_ulong,
        flags: c_ulong
    ) -> isize;

    safe fn process_vm_writev(
        pid: c_int,
        local_iov: *const iovec, liovcnt: c_ulong,
        remote_iov: *const iovec, riovcnt: c_ulong,
        flags: c_ulong
    ) -> isize;
}