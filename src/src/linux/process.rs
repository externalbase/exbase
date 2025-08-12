use std::{ffi::c_int, fs::{self, File}, io::{BufRead, BufReader}};

use crate::{LibraryInfo, ProcessInfo};

impl ProcessInfo {
    pub fn attach<M: MemoryAccessor>(self, memory: M) -> Process<M> {
        Process {
            info: self,
            memory,
        }
    }
    pub(crate) fn from_pid(pid: u32) -> Option<Self> {
        if !is_alive(pid as i32) {
            return None;
        }
        Some(Self {
            pid,
            name: fs::read_to_string(format!("/proc/{pid}/comm")).ok()?.trim_end().to_owned(),
            cmd: fs::read_to_string(format!("/proc/{pid}/cmdline")).ok()?.trim_end().to_owned(),
            exe: fs::read_link(format!("/proc/{pid}/exe")).ok()?.to_string_lossy().into_owned()
        })
    }

    pub fn get_libraries(&self) -> Option<Vec<LibraryInfo>> {
        if !is_alive(self.pid as i32) {
            return None;
        }
        let mut result = Vec::new();
        let maps = File::open(format!("/proc/{}/maps", self.pid)).ok()?;
        for line in BufReader::new(maps).lines() {
            if let Ok(line) = line {
                if let Some(segment) = parse_segment(line.trim_end().to_owned()) {
                    result.push(segment);
                }
            }
        }
        Some(result)
    }
}

fn parse_segment(line: String) -> Option<LibraryInfo> {
    let parts: Vec<&str> = line.splitn(6, ' ').collect();
    if parts.len() == 6 {
        let mut iterator = parts.iter();
        let addr_range = iterator.next()?;          // split '-'
        let perms = iterator.next()?;               // Permissions (rwxp)
        let offset = iterator.next()?;
        if offset.parse::<i32>().ok()? == 0i32 {
            _ = iterator.next()?;                   // dev number
            _ = iterator.next()?;                   // inode
            let path = iterator.next()?.trim();
            if path.starts_with('/') && path.contains(".so") {
                
                let addr_range: Vec<_> = addr_range.split('-').collect();
                let start = usize::from_str_radix(addr_range.get(0)?, 16).ok()?;
                let end = usize::from_str_radix(addr_range.get(1)?, 16).ok()?;
                return  Some(LibraryInfo {
                    bin: path.to_owned(),
                    address: start,
                    size: end - start,
                    perms: perms.to_string()
                });
            }
        }
    }
    None
}