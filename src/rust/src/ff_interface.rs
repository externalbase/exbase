use std::{ffi::{c_char, c_int, c_uint, c_void, CStr}, mem};

use crate::ProcessInfo;

pub type CProcessInfo = *mut c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_process_info_list(name: *const c_char, out_len: *mut c_int) -> CProcessInfo {
    if let Some(name) = rstr(name) {
        if let Some(vec_r) = crate::get_process_info_list(name) {
            // println!("RUST: {:?}", vec_r);
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
pub unsafe extern "C" fn process_info_pid(p_proc: CProcessInfo) -> c_uint {
    unsafe {
        let proc: &ProcessInfo = &*(p_proc as *const ProcessInfo);
        proc.pid
    }
}

fn rstr(p_str: *const c_char) -> Option<&'static str> {
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