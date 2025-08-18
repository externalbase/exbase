use super::*;

#[cfg(target_os = "linux")]
const PROCESS_MAME: &str = "bash";
#[cfg(target_os = "windows")]
const PROCESS_MAME: &str = "explorer.exe";

#[test]
fn test1() {
    let proc_info;
    proc_info = get_process_info_list(PROCESS_MAME)
        .unwrap()
        .into_iter()
        .next()
        .unwrap();

    proc_info.get_modules()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
}