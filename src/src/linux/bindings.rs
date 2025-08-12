unsafe extern "C" {
    safe fn kill(pid: c_int, sig: c_int) -> c_int;
}