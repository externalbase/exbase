#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("Windows or Linux only");

#[cfg_attr(target_os = "linux", path = "linux/process.rs")]
#[cfg_attr(target_os = "windows", path = "windows/process.rs")]
mod process;
#[cfg_attr(target_os = "linux", path = "linux/memory.rs")]
#[cfg_attr(target_os = "windows", path = "windows/memory.rs")]
mod memory;
#[cfg_attr(target_os = "linux", path = "linux/bindings.rs")]
#[cfg_attr(target_os = "windows", path = "windows/bindings.rs")]
pub(crate) mod bindings;
#[cfg(test)]
mod tests;
#[cfg(feature = "ffi")]
mod ff_interface;
pub mod error;
pub mod patern_scanner;

use error::Result;
use std::{fs::File};

pub trait MemoryAccessor {
    fn read_buffer(&self, buf: &mut [u8], addr: usize);
    #[cfg(not(feature = "read_only"))]
    fn write_buffer(&self, buf: &[u8], addr: usize);

    fn read<T: Copy>(&self, addr: usize) -> T {
        let mut buf = vec![0u8; std::mem::size_of::<T>()];
        self.read_buffer(&mut buf, addr);
        unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const T) }
    }

    fn read_string(&self, addr: usize, max_len: usize) -> String {
        let mut buf = vec![0u8; max_len];
        self.read_buffer(&mut buf, addr);
        
        let slice = match buf.iter().position(|&b| b == 0) {
            Some(n) => &buf[..n],
            None => &buf[..],
        };
        String::from_utf8_lossy(slice).into_owned()
    }
    
    #[cfg(not(feature = "read_only"))]
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

pub struct SysMem {
    pid: i32,
    #[cfg(target_os = "windows")]
    handle: HANDLE,
}
#[cfg(target_os = "linux")]
pub struct StreamMem {
    pub(crate) mem: File
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    #[cfg(target_os = "windows")]
    handle: HANDLE,
    pid: u32,
    name: String,
    exe: String,
}

#[derive(Debug, Clone)]
pub struct LibraryInfo {
    name: String,
    address: usize,
    size: usize,
    perms: String,
}

impl ProcessInfo {
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn exe(&self) -> String {
        self.exe.clone()
    }
}

impl LibraryInfo {
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn address(&self) -> usize {
        self.address
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn can_read(&self) -> bool {
        &self.perms[0..1] == "r"
    }

    pub fn can_write(&self) -> bool {
        &self.perms[1..2] == "w"
    }
}

impl<M: MemoryAccessor> Process<M> {
    pub fn get_info(&self) -> ProcessInfo {
        self.info.clone()
    }
}

#[cfg(target_os = "linux")]
pub fn get_process_info_list<S: AsRef<str>>(name: S) -> Result<Vec<ProcessInfo>> {
    use std::{fs, io::Read};
    let mut result: Vec<ProcessInfo> = Vec::new();
    for entry in fs::read_dir("/proc")? {
        if let Ok(entry) = entry {
            let pid = match entry.file_name().to_string_lossy().parse::<u32>() {
                Ok(r) => r,
                Err(_) => continue,
            };
            let mut buf_comm = String::new();
            if fs::File::open(format!("/proc/{pid}/comm"))
                .and_then(|mut f| f.read_to_string(&mut buf_comm))
                .is_ok()
            {
                if buf_comm.trim_end() == name.as_ref() {
                    result.push(ProcessInfo::from_pid(pid)?);
                }
            }
        }
    }
    Ok(result)
}

#[cfg(target_os = "windows")]
pub fn get_process_info_list<S: AsRef<str>>(name: S) -> Result<Vec<ProcessInfo>> {
    let mut result: Vec<ProcessInfo> = Vec::new();
    let mut proc_entry = PROCESSENTRY32W::default();
    let h_snap  = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if h_snap <= 0 as _ {
        return Err(Error::os("CreateToolhelp32Snapshot"))
    }
    let _guard = HandleGuard(h_snap);
    proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
    if (unsafe { Process32FirstW(h_snap, &mut proc_entry) } == 0) {
        return Err(Error::os("Process32FirstW"));
    }
    loop {
        let proc_name = String::from_utf16_lossy(&proc_entry.szExeFile).trim_end_matches('\0').to_owned();
            if proc_name == name.as_ref() {
                result.push(ProcessInfo::new(proc_entry.th32ProcessID, name.as_ref().to_string())?);
            }
        if unsafe { Process32NextW(h_snap, &mut proc_entry) } == 0 {
            break;
        }
    }

    Ok(result)
}