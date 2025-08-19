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

use error::Result;
#[cfg(target_os = "linux")]
use std::{fs::File};
#[cfg(target_os = "windows")]
use bindings::*;

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
pub struct ModuleInfo {
    name: String,
    address: usize,
    size: usize,
    perms: String,
}

pub struct Pattern {
    pub bytes: Vec<u8>,
    pub mask: Vec<u8>,
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

impl ModuleInfo {
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

impl Pattern {
    /// # Example
    /// ```
    /// Pattern::new("00 FF 0? F? ?? ?").unwrap();
    /// ```
    pub fn new(pattern: &str) -> Option<Self> {
        let mut result = Pattern { bytes: Vec::new(), mask: Vec::new() };
        let mut first_solid_found = false;
        if pattern.trim().is_empty() {
            return  None;
        }
        for strb in pattern.split_whitespace() {
            if strb.len() > 2 {
                return None;
            }
            let any_wild = strb.contains('?');
            let full_wild = strb == "??" || (any_wild && strb.len() == 1);

            if !first_solid_found && !full_wild {
                first_solid_found = true;
            }
            if full_wild {
                result.bytes.push(0x00);
                result.mask.push(0x00);
                continue;
            }

            if !any_wild {
                let byte = u8::from_str_radix(strb, 16).ok()?;
                result.bytes.push(byte);
                result.mask.push(0xFF);
                continue;
            }

            let mut chars: Vec<char> = strb.chars().collect();
            if chars[0] == '?' {
                chars[0] = '0';
                let hex = chars.iter().collect::<String>();
                let byte = u8::from_str_radix(&hex, 16).ok()?;
                result.bytes.push(byte);
                result.mask.push(0x0F);
            } else {
                chars[1] = '0';
                let hex = chars.iter().collect::<String>();
                let byte = u8::from_str_radix(&hex, 16).ok()?;
                result.bytes.push(byte);
                result.mask.push(0xF0);
            }
        }

        Some(result)
    }

    fn build_shift_table(&self) -> [usize; 256] {
        let m = self.bytes.len();
        let mut shift = [m; 256];
        for i in 0..m-1 {
            let b = self.bytes[i] & self.mask[i];
            shift[b as usize] = m - 1 - i;
        }
        shift
    }

    pub fn scan(&self, buf: &[u8], first_only: bool) -> Vec<usize> {
        let m = self.bytes.len();
        let n = buf.len();
        if m == 0 || n < m { return Vec::new(); }

        let shift = self.build_shift_table();
        let mut result = Vec::new();
        let mut pos = 0;

        while pos <= n - m {
            let last_buf = buf[pos + m - 1] & self.mask[m - 1];
            let last_pat = self.bytes[m - 1] & self.mask[m - 1];
            if last_buf == last_pat {
                let mut matched = true;
                for i in 0..m-1 {
                    if (buf[pos+i] & self.mask[i]) != (self.bytes[i] & self.mask[i]) {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    result.push(pos);
                    if first_only {
                        return result;
                    }
                }
                pos += 1;
            } else {
                pos += shift[last_buf as usize];
            }
        }
        result
    }
}

pub fn relative_address(mem: &impl MemoryAccessor, pattern_addr: usize, offset: usize, inst_length: usize) -> usize 
{
    let rip_rel = mem.read::<i32>(pattern_addr + offset);
    pattern_addr.wrapping_add(inst_length).wrapping_add(rip_rel as usize)
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
        return Err(crate::error::Error::os("CreateToolhelp32Snapshot"))
    }
    let _guard = HandleGuard(h_snap);
    proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
    if (unsafe { Process32FirstW(h_snap, &mut proc_entry) } == 0) {
        return Err(crate::error::Error::os("Process32FirstW"));
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