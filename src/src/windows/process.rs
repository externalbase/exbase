use crate::error::{Error, Result};
use crate::{bindings::*, LibraryInfo, MemoryAccessor, Process, ProcessInfo};

impl ProcessInfo {
    pub fn attach<M: MemoryAccessor>(self, memory: M) -> Process<M> {
        Process {
            info: self,
            memory,
        }
    }

    pub fn new(pid: u32, name: String) -> Result<Self> {
        let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid) };
        if handle <= 0 as _ {
            return Err(Error::os("OpenProcess"));
        }

        let mut exe_buf = [0u16; MAX_PATH as usize + 1];
        unsafe { GetModuleFileNameExW(handle, HANDLE::default(), exe_buf.as_mut_ptr(), exe_buf.len() as u32) };

        Ok(Self {
            handle,
            pid,
            name,
            exe: String::from_utf16_lossy(&exe_buf).to_owned(),
        })
    }

    pub fn get_libraries(&self) -> Result<Vec<LibraryInfo>> {
        if self.handle <= 0 as _ {
            return  Err(Error::os("Process handle is null"));
        }

        let mut mods = vec![HMODULE::default(); 512];
        let mut cb_needed = 0u32;
        let suc = unsafe {
            EnumProcessModulesEx(
                self.handle,
                mods.as_mut_ptr() as *mut _,
                (mods.len() * std::mem::size_of::<usize>()) as u32,
                &mut cb_needed,
                LIST_MODULES_ALL,
            )
        };
        if suc == 0 {
            unsafe { CloseHandle(self.handle) };
            return Err(Error::os("EnumProcessModulesEx"));
        }

        mods.truncate( (cb_needed as usize) / std::mem::size_of::<usize>());
        let mut result = Vec::new();

        for &hmod in &mods {
            let mut name_buf = [0u16; MAX_PATH as usize];
            let name_len = unsafe {
                GetModuleBaseNameW(
                    self.handle,
                    hmod as _,
                    name_buf.as_mut_ptr(),
                    MAX_PATH,
                )
            };

            let module_name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
            let mut mod_info = MODULEINFO { lpBaseOfDll: std::ptr::null_mut(), SizeOfImage: 0, EntryPoint: std::ptr::null_mut() };
            if unsafe { GetModuleInformation(self.handle, hmod as _, &mut mod_info, std::mem::size_of::<MODULEINFO>() as u32) == 0 } {
                continue;
            }

            result.push(
                LibraryInfo {
                    bin: module_name,
                    address: mod_info.lpBaseOfDll as usize,
                    size: mod_info.SizeOfImage as usize,
                    #[cfg(feature = "read_only")]
                    perms: "r---".to_owned(),
                    #[cfg(not(feature = "read_only"))]
                    perms: "rw--".to_owned(),
                }
            );
        }

        Ok(result)
    }
}

impl Drop for ProcessInfo {
    fn drop(&mut self) {
        if self.handle != 0 as HANDLE {
            unsafe { CloseHandle(self.handle) };
        }
    }
}