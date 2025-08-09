use std::{ffi::{c_char, c_int, c_uint, c_void, CStr, CString}, mem};

use crate::ProcessInfo;
use ffi_utils::*;

pub type CProcessInfo = *mut c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_process_info_list(name: *const c_char, out_len: *mut c_int) -> CProcessInfo {
    if let Some(name) = rstr(name) {
        if let Some(vec_r) = crate::get_process_info_list(name) {
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
    unsafe { *out_len = 0 }
    return std::ptr::null_mut();
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
pub unsafe extern "C" fn free_cstring(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe { _ = CString::from_raw(s); };
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