use std::ffi::c_void;

#[cfg_attr(target_os = "linux", path = "linux/process.rs")]
#[cfg_attr(target_os = "windows", path = "linux/process.rs")]
mod process;
#[cfg_attr(target_os = "linux", path = "linux/memory.rs")]
#[cfg_attr(target_os = "windows", path = "linux/memory.rs")]
pub mod memory;
#[cfg(test)]
mod tests;

pub trait Memory {
    fn read<T: Copy>(&self, address: *mut c_void) -> T;
    fn read_buffer(&self, address: *mut c_void, len: usize) -> Vec<u8>;
}

pub struct Process<M: Memory> {
    pub(crate) info: ProcessInfo,
    pub memory: M,
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
    pub fn attach<M: Memory>(self, memory: M) -> Process<M> {
        Process {
            info: self,
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