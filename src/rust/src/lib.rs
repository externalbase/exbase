#[cfg_attr(target_os = "linux", path = "linux/process.rs")]
#[cfg_attr(target_os = "windows", path = "linux/process.rs")]
mod process;
#[cfg_attr(target_os = "linux", path = "linux/memory.rs")]
#[cfg_attr(target_os = "windows", path = "linux/memory.rs")]
pub mod memory;
#[cfg(test)]
mod tests;

use std::fs::File;

pub trait MemoryAccessor {
    fn read_buffer(&self, buf: &mut [u8], addr: usize);
    fn write_buffer(&self, buf: &[u8], addr: usize);

    fn read<T: Copy>(&self, addr: usize) -> T {
        let mut buf = vec![0u8; std::mem::size_of::<T>()];
        self.read_buffer(&mut buf, addr);
        unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const T) }
    }

    /// max_len: 256
    fn read_string(&self, addr: usize, max_len: usize) -> String {
        let mut buf = vec![0u8; max_len];
        self.read_buffer(&mut buf, addr);
        
        let slice = match buf.iter().position(|&b| b == 0) {
            Some(n) => &buf[..n],
            None => &buf[..],
        };
        String::from_utf8_lossy(slice).into_owned()
    }
    
    fn write<T: Copy>(&self, addr: usize, value: T) {
        let mut buf = vec![0u8; std::mem::size_of::<T>()];
        let ptr = &value as *const T;
        unsafe {
            std::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr() as *mut T, 1);
        }
        self.write_buffer(&mut buf, addr);
    }
    
}
pub struct Process<M: MemoryAccessor> {
    pub(crate) info: ProcessInfo,
    pub memory: M,
}

//     SysCall,
pub struct SystemMem;
//     VFile
pub struct StreamMem {
    pub(crate) mem: File
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pid: u32,
    name: String,
    cmd: String,
    exe: String,
}

#[derive(Debug, Clone)]
pub struct LibraryInfo {
    bin: String,
    address: usize,
    size: usize,
    perms: String,
}


impl ProcessInfo {
    pub fn attach<M: MemoryAccessor>(&self, memory: M) -> Process<M> {
        Process {
            info: self.clone(),
            memory,
        }
    }

    pub fn is_alive(&self) -> bool {
        process::is_alive(self.pid as i32)
    }

    pub fn get_pid(&self) -> u32 {
        self.pid
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_executable(&self) -> String {
        self.exe.clone()
    }

    pub fn get_cmd(&self) -> String {
        self.cmd.clone()
    }
}

impl LibraryInfo {
    pub fn get_bin(&self) -> String {
        self.bin.to_string()
    }

    pub fn get_address(&self) -> usize {
        self.address
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn can_read(&self) -> bool {
        &self.perms[0..1] == "r"
    }

    pub fn can_write(&self) -> bool {
        &self.perms[1..2] == "w"
    }
}