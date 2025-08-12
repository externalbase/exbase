use std::{fs::{self, File}, io::{BufRead, BufReader}};

use crate::{bindings, LibraryInfo, MemoryAccessor, Process, ProcessInfo, error::{Result, Error}};

impl ProcessInfo {
    pub fn attach<M: MemoryAccessor>(self, memory: M) -> Process<M> {
        Process {
            info: self,
            memory,
        }
    }
    pub(crate) fn from_pid(pid: u32) -> Result<Self> {
        if !is_alive(pid as i32) {
            return Err(Error::other("Process is inactive"));
        }
        Ok(Self {
            pid,
            name: fs::read_to_string(format!("/proc/{pid}/comm"))?.trim_end().to_owned(),
            cmd: fs::read_to_string(format!("/proc/{pid}/cmdline"))?.trim_end().to_owned(),
            exe: fs::read_link(format!("/proc/{pid}/exe"))?.to_string_lossy().into_owned()
        })
    }

    pub fn get_libraries(&self) -> Result<Vec<LibraryInfo>> {
        if !is_alive(self.pid as i32) {
            return Err(Error::other("Process is inactive"));
        }
        let mut result = Vec::new();
        let maps = File::open(format!("/proc/{}/maps", self.pid))?;
        for line in BufReader::new(maps).lines() {
            if let Ok(line) = line {
                if let Some(segment) = parse_segment(line.trim_end().to_owned()) {
                    result.push(segment);
                }
            }
        }
        Ok(result)
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
            if path == "[heap]" || path == "[stack]" || (path.starts_with('/') && path.contains(".so")) {
                
                let addr_range: Vec<_> = addr_range.split('-').collect();
                let start = usize::from_str_radix(addr_range.get(0)?, 16).ok()?;
                let end = usize::from_str_radix(addr_range.get(1)?, 16).ok()?;
                return  Some(LibraryInfo {
                    name: path.to_owned(),
                    address: start,
                    size: end - start,
                    perms: perms.to_string()
                });
            }
        }
    }
    None
}

pub fn is_alive(pid: i32) -> bool {
    bindings::kill(pid, 0) == 0
}