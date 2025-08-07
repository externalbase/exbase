use exbase::ProcessInfo;

pub fn main() {
    let processes = ProcessInfo::processes_by_name("cs2").expect("Failed to get processes");
    let process_info = processes.iter().next().expect("Not found");

    for lib in process_info.get_libraries().unwrap() {
        if lib.can_read() {
            println!("Address: 0x{:x}\tLocation: {}", lib.get_address(), lib.get_bin())
        }
    }


    let pid = process_info.get_pid();
    // let process = process_info.attach(); // enum?
}

pub fn e2() {
    let _proc = ProcessInfo::from_pid(1480).expect("Not found or permission denied");
}