use super::*;

const TEST_PROCESS_NAME: &str = "firefox.exe";

#[test]
fn test1() {
    let proc_vec = get_process_info_list(TEST_PROCESS_NAME).unwrap();
    let proc_info = proc_vec.get(0).unwrap();
    println!("len: {}", proc_vec.len());
    println!("{}\n{}\n{}\n{}", proc_info.cmd, proc_info.exe, proc_info.name, proc_info.pid);
    panic!();
}
