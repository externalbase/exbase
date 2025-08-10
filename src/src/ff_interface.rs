use std::{ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr, CString}, mem, ptr};

use crate::{LibraryInfo, ProcessInfo};
use ffi_utils::*;

pub type CProcessInfo = *mut c_void;
pub type CLibraryInfo = *mut c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_process_info_list(name: *const c_char, out_len: *mut c_int) -> CProcessInfo {
    if let Some(name) = rstr(name) {
        if let Some(vec_r) = crate::get_process_info_list(name) {
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
    if let Some(vec_r) = proc.get_libraries() {
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
pub unsafe extern "C" fn library_info_bin(p_lib: CLibraryInfo) -> *const c_char {
    throw_if_null(p_lib);
    let lib: &LibraryInfo = deref(p_lib);
    CString::new(lib.bin.clone()).unwrap().into_raw()
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_cstring(s: *const c_char) {
    if s.is_null() {
        return;
    }
    unsafe { _ = CString::from_raw(s as *mut c_char); };
}

pub(crate) mod ffi_utils {
    use std::ffi::{c_char, c_void, CStr};

    pub fn throw_if_null(ptr: *mut c_void) {
        if ptr.is_null() {
            panic!("null pointer exception");
        }
    }

    pub fn deref<T>(p: *mut c_void) -> &'static T {
        unsafe { &*(p as *const T) }
    }

    pub fn rstr(p_str: *const c_char) -> Option<&'static str> {
        if p_str.is_null() {
            eprint!("str is null");
            return None;
        }
        let c_str = unsafe { CStr::from_ptr(p_str) }.to_str();
        match c_str {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("process_by_name -> {e}");
                None
            },
        }
    }
}