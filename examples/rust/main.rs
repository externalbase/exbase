use std::sync::Mutex;

use exbase::{MemoryAccessor, ProcessInfo, SysMem};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MyStruct {
    pub num: i32,
    pub long_text: usize,
    pub short_text: usize,
    pub num2: u16,
    pub _padding: [u8; 16],
    pub num3: i8,
}

static HEAP_ADDR: Mutex<usize> = Mutex::new(0);

pub fn main() {
    let proc_info_list = exbase::get_process_info_list("ABC123").expect("Failed to get processes");
    let out_len = proc_info_list.len();

    if out_len == 0 {
        eprintln!("Не найдено ни одного процесса");
        std::process::exit(1);
    }
    if out_len > 1 {
        eprintln!("Найдено {} процессов", out_len);
        for i in 0..out_len {
            eprintln!("{}. PID: {}", i, proc_info_list[i].pid());
        }
        std::process::exit(1);
    }

    let proc_info = proc_info_list.into_iter().next().unwrap();

    print_process_info(&proc_info);
    print_libraries(&proc_info);
    
    let pid = proc_info.pid();
    let mem = SysMem::new(pid).unwrap();

    read_write_field(&mem);
    read_write_struct(&mem);
}

fn print_process_info(proc_info: &ProcessInfo) {
    println!("PID: {}", proc_info.pid());
    println!("Name: {}", proc_info.name());
    println!("Executable: {}\n", proc_info.exe());
}

fn print_libraries(proc_info: &ProcessInfo) {
    let libraries = proc_info.get_libraries().expect("Не удалось получить библиотеки");

    for lib in libraries {
        println!("Name: {}", lib.name());
        println!("Address: 0x{:x}", lib.address());
        println!("Size: {} bytes\n", lib.size());

        if lib.name() == "[heap]" {
            let mut addr = HEAP_ADDR.lock().unwrap();
            *addr = lib.address();
        }
    }
}

fn read_write_field(mem: &impl MemoryAccessor) {
    let addr: usize = *HEAP_ADDR.lock().unwrap() + 0x2a0;
    let num2 = mem.read::<i32>(addr + 0x18) * -1;
    mem.write::<i32>(addr + 0x18, num2);
}

fn read_write_struct(mem: &impl MemoryAccessor) {
    let addr: usize = *HEAP_ADDR.lock().unwrap() + 0x2a0;
    let mut my_struct: MyStruct = mem.read(addr);
    my_struct.num += 3;
    my_struct.num3 = my_struct.num3.wrapping_mul(2);
    mem.write(addr, my_struct);

    let short_text = mem.read_string(my_struct.short_text, 256);
    println!("short_text: {}, text len: {}", short_text, short_text.len());
    mem.write_buffer(b":p\0", my_struct.long_text);
}