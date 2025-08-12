use std::{ffi::{c_char, c_int, c_uint, c_void, CString}, mem, ptr};

use crate::{LibraryInfo, MemoryAccessor, Process, ProcessInfo, SysMem};
#[cfg(target_os = "linux")]
use crate::StreamMem;
use ffi_utils::*;
use crate::error::ErrorFFI;

pub type Result<T> = std::result::Result<T, ErrorFFI>;

pub type CProcessInfo = *mut c_void;
pub type CLibraryInfo = *mut c_void;
pub type CProcess = *mut c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_process_info_list(name: *const c_char, out_len: *mut c_int) -> CProcessInfo {
    if let Ok(name) = rstr(name) {
        if let Ok(vec_r) = crate::get_process_info_list(name) {
            if vec_r.len() > 0 {
                let mut vec_c = Vec::new();
                for p in vec_r {
                    let p_proc = Box::into_raw(Box::new(p));
                    vec_c.push(p_proc);
                }
                vec_c.shrink_to_fit();
                let ptr = vec_c.as_mut_ptr() as CProcessInfo;
                unsafe { *out_len = vec_c.len() as c_int };
                mem::forget(vec_c);
                return ptr;
            }
        }
    }
    unsafe { *out_len = 0 }
    return ptr::null_mut();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_info_get_libraries(p_proc: CProcessInfo, out_len: *mut c_int) -> CLibraryInfo {
    throw_if_null(p_proc);
    let proc: &ProcessInfo = deref(p_proc);
    if let Ok(vec_r) = proc.get_libraries() {
        if vec_r.len() > 0 {
            let mut vec_c = Vec::new();
            for lib in vec_r {
                let p_lib = Box::into_raw(Box::new(lib));
                vec_c.push(p_lib);
            }
            vec_c.shrink_to_fit();
            let ptr = vec_c.as_mut_ptr() as CLibraryInfo;
            unsafe { *out_len = vec_c.len() as c_int };
            mem::forget(vec_c);
            return ptr;
        }
    }
    unsafe { *out_len = 0; };
    ptr::null_mut()
}

/**
 * ProcessInfo
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_process_info(p_proc: CProcessInfo) {
    throw_if_null(p_proc);
    drop(unsafe { Box::from_raw(p_proc as CProcessInfo) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_info_pid(p_proc: CProcessInfo) -> c_uint {
    throw_if_null(p_proc);
    let proc: &ProcessInfo = deref(p_proc);
    proc.pid
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_info_name(p_proc: CProcessInfo) -> *const c_char {
    throw_if_null(p_proc);
    let proc: &ProcessInfo = deref(p_proc);
    CString::new(proc.name.clone()).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_info_cmd(p_proc: CProcessInfo) -> *const c_char {
    throw_if_null(p_proc);
    let proc: &ProcessInfo = deref(p_proc);
    CString::new(proc.cmd.chars().filter(|&c| c != '\0').collect::<String>()).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_info_exe(p_proc: CProcessInfo) -> *const c_char {
    throw_if_null(p_proc);
    let proc: &ProcessInfo = deref(p_proc);
    CString::new(proc.exe.clone()).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_info_attach(p_proc: CProcessInfo) -> CProcess {
    throw_if_null(p_proc);
    let proc_info: &ProcessInfo = deref(p_proc);
    if let Ok(m) = SysMem::new(proc_info.pid) {
        let proc = proc_info.clone().attach(m);
        return Box::into_raw(Box::new(proc)) as CProcess;
    }
    ptr::null_mut()
}

#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_info_attach_vfile(p_proc: CProcessInfo) -> CProcess {
    throw_if_null(p_proc);
    let proc_info: &ProcessInfo = deref(p_proc);
    if let Ok(m) = StreamMem::new(proc_info.pid) {
        let proc = proc_info.clone().attach(m);
        return Box::into_raw(Box::new(proc)) as CProcess;
    }
    ptr::null_mut()
}

/**
 * LibraryInfo
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn library_info_name(p_lib: CLibraryInfo) -> *const c_char {
    throw_if_null(p_lib);
    let lib: &LibraryInfo = deref(p_lib);
    CString::new(lib.name.clone()).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn library_info_perms(p_lib: CLibraryInfo) -> *const c_char {
    throw_if_null(p_lib);
    let lib: &LibraryInfo = deref(p_lib);
    CString::new(lib.perms.clone()).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn library_info_address(p_lib: CLibraryInfo) -> usize {
    throw_if_null(p_lib);
    let lib: &LibraryInfo = deref(p_lib);
    lib.address
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn library_info_size(p_lib: CLibraryInfo) -> usize {
    throw_if_null(p_lib);
    let lib: &LibraryInfo = deref(p_lib);
    lib.size
}

/**
 * Process
 */

#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_write_buffer_vfile(proc: CProcess, buf: *const u8, size: usize, addr: usize) {
    throw_if_null(proc);
    let proc: &Process<StreamMem> = deref(proc);
    proc.memory.write_buffer(unsafe { std::slice::from_raw_parts(buf, size) }, addr);
}

#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_read_buffer_vfile(proc: CProcess, buf: *mut u8, size: usize, addr: usize) {
    throw_if_null(proc);
    let proc: &Process<StreamMem> = deref(proc);
    proc.memory.read_buffer(unsafe { std::slice::from_raw_parts_mut(buf, size) }, addr);
}

#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_read_string_vfile(proc: CProcess, max_len: usize, addr: usize) -> *const c_char {
    throw_if_null(proc);
    let proc: &Process<StreamMem> = deref(proc);
    let s = proc.memory.read_string(addr, max_len);
    CString::new(s.chars().filter(|&c| c != '\0').collect::<String>()).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_write_buffer(proc: CProcess, buf: *const u8, size: usize, addr: usize) {
    throw_if_null(proc);
    let proc: &Process<SysMem> = deref(proc);
    proc.memory.write_buffer(unsafe { std::slice::from_raw_parts(buf, size) }, addr);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_read_buffer(proc: CProcess, buf: *mut u8, size: usize, addr: usize) {
    throw_if_null(proc);
    let proc: &Process<SysMem> = deref(proc);
    proc.memory.read_buffer(unsafe { std::slice::from_raw_parts_mut(buf, size) }, addr);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_read_string(proc: CProcess, max_len: usize, addr: usize) -> *const c_char {
    throw_if_null(proc);
    let proc: &Process<SysMem> = deref(proc);
    let s = proc.memory.read_string(addr, max_len);
    CString::new(s.chars().filter(|&c| c != '\0').collect::<String>()).unwrap().into_raw()
}

/**
 * free
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_process_info_list(p_proc: CProcessInfo, len: c_int) {
    let ulen = len as usize;
    let vec_c = unsafe { Vec::from_raw_parts(p_proc as *mut CProcessInfo, ulen, ulen) };
    unsafe {
        for p in vec_c {
            drop(Box::from_raw(p as *mut ProcessInfo));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_library_info_list(p_lib: CLibraryInfo, len: c_int) {
    let ulen = len as usize;
    let vec_c = unsafe { Vec::from_raw_parts(p_lib as *mut CLibraryInfo, ulen, ulen) };
    unsafe {
        for p in vec_c {
            drop(Box::from_raw(p as *mut LibraryInfo));
        }
    }
}

#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_process_vfile(proc: CProcess) {
    if proc.is_null() {
        return;
    }
    unsafe {
        let proc: Box<Process<StreamMem>> = Box::from_raw(proc as *mut Process<_>);
        drop(proc);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_process(proc: CProcess) {
    if proc.is_null() {
        return;
    }
    unsafe {
        let proc: Box<Process<SysMem>> = Box::from_raw(proc as *mut Process<_>);
        drop(proc);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_cstring(s: *const c_char) {
    if s.is_null() {
        return;
    }
    unsafe { _ = CString::from_raw(s as *mut c_char); };
}

pub(crate) mod ffi_utils {
    use std::ffi::{c_char, c_void, CStr};
    use crate::error::ErrorFFI;

    pub fn throw_if_null(ptr: *mut c_void) {
        if ptr.is_null() {
            panic!("null pointer exception");
        }
    }

    pub fn deref<T>(p: *mut c_void) -> &'static T {
        unsafe { &*(p as *const T) }
    }

    pub fn rstr(p_str: *const c_char) -> Result<&'static str, ErrorFFI> {
        if p_str.is_null() {
            return Err(ErrorFFI::NullPointer { obj: "*const char".to_owned() });
        }
        unsafe { CStr::from_ptr(p_str) }.to_str().map_err(ErrorFFI::Utf8Error)
    }
}