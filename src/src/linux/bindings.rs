use std::ffi::{c_int, c_ulong, c_void};

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize, // size_t
}

unsafe extern "C" {
    pub(crate) safe fn kill(pid: c_int, sig: c_int) -> c_int;

    pub(crate) safe fn process_vm_readv(
        pid: c_int,
        local_iov: *const iovec, liovcnt: c_ulong,
        remote_iov: *const iovec, riovcnt: c_ulong,
        flags: c_ulong
    ) -> isize;

    pub(crate) safe fn process_vm_writev(
        pid: c_int,
        local_iov: *const iovec, liovcnt: c_ulong,
        remote_iov: *const iovec, riovcnt: c_ulong,
        flags: c_ulong
    ) -> isize;
}