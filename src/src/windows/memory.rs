use std::ffi::c_void;
use crate::{bindings::*, error::Error, error::Result, MemoryAccessor, SysMem};

impl MemoryAccessor for SysMem {
    fn read_buffer(&self, buf: &mut [u8], addr: usize) {
        let suc = unsafe {
            let mut bytesread: usize = 0;
            ReadProcessMemory(
                self.handle, addr as *mut c_void,
                buf.as_mut_ptr() as _,
                buf.len(),
                &mut bytesread
            ) 
        };
        if suc == 0 {
            let err = std::io::Error::last_os_error();
            eprintln!("process_vm_readv: {}", err);
        }
    }

    #[cfg(not(feature = "read_only"))]
    fn write_buffer(&self, buf: &[u8], addr: usize) {
        let suc = unsafe {
            let mut bytesread: usize = 0;
            WriteProcessMemory(
                self.handle, addr as *mut c_void,
                buf.as_ptr() as *const c_void,
                buf.len(),
                &mut bytesread
            ) 
        };
        if suc == 0 {
            let err = std::io::Error::last_os_error();
            eprintln!("process_vm_readv: {}", err);
        }
    }
}

impl SysMem {
    pub fn new(pid: u32) -> Result<Self> {
        #[cfg(feature = "read_only")]
        let options = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;
        #[cfg(not(feature = "read_only"))]
        let options = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_VM_WRITE;
        let h = unsafe { OpenProcess(options, 0, pid) };
        if h <= 0 as HANDLE {
            return Err(Error::os("OpenProcess"));
        }
        Ok(Self {
            pid: pid as i32,
            handle: h,
        })
    }
}

impl Drop for SysMem {
    fn drop(&mut self) {
        if self.handle != 0 as HANDLE {
            unsafe { CloseHandle(self.handle); }
        }
    }
}