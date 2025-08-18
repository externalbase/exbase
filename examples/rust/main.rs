use std::sync::Mutex;

use exbase::{relative_address, MemoryAccessor, Pattern, ProcessInfo, SysMem};

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
static SCAN_RANGE_START: Mutex<usize> = Mutex::new(0usize);
static SCAN_RANGE_SIZE: Mutex<usize> = Mutex::new(0usize);

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
    print_modules(&proc_info);

    let mem = SysMem::new(proc_info.pid()).unwrap();

    // mov    rax,QWORD PTR [rip+0x2a51]        # 0x403040 <my_struct_ptr>
    // mov    eax,DWORD PTR [rax]
    // mov    rdx,QWORD PTR [rbp-0x8]
    let pat = Pattern::new("48 8b 05 ? ? ? ? 8b ?").unwrap();

    // 00400000-00401000 r-xp 00000000 00:23 1730705                            /path/to/ABC123
    // ...
    // 00403000-00404000 rw-p 00002000 00:23 1730705                            /path/to/ABC123
    let scan_start = SCAN_RANGE_START.lock().unwrap();

    let mut buf = vec!(0u8; *SCAN_RANGE_SIZE.lock().unwrap());

    mem.read_buffer(&mut buf, *scan_start);
    
    let pattern_offset = pat.scan(&buf, false).into_iter().next().expect("not found");
    
    let my_struct_ptr = relative_address(&mem, *scan_start + pattern_offset, 3, 7);

    read_write_field(&mem, my_struct_ptr);
    read_write_struct(&mem, my_struct_ptr);
}

fn print_process_info(proc_info: &ProcessInfo) {
    println!("PID: {}", proc_info.pid());
    println!("Name: {}", proc_info.name());
    println!("Executable: {}\n", proc_info.exe());
}

fn print_modules(proc_info: &ProcessInfo) {
    let modules = proc_info.get_modules().expect("Не удалось получить библиотеки");

    for r#mod in modules {
        println!("Name: {}", r#mod.name());
        println!("Address: 0x{:x}", r#mod.address());
        println!("Size: {} bytes\n", r#mod.size());

        if r#mod.name() == "ABC123" {
            *SCAN_RANGE_START.lock().unwrap() = r#mod.address();
            *SCAN_RANGE_SIZE.lock().unwrap() = r#mod.size();
        }
    }
}

fn read_write_field(mem: &impl MemoryAccessor, my_struct_ptr: usize) {
    let addr = mem.read::<usize>(my_struct_ptr);
    let num2 = mem.read::<i32>(addr + 0x18) * -1;
    mem.write::<i32>(addr + 0x18, num2);
}

fn read_write_struct(mem: &impl MemoryAccessor, my_struct_ptr: usize) {
    let addr = mem.read::<usize>(my_struct_ptr);
    let mut my_struct: MyStruct = mem.read(addr);
    my_struct.num += 3;
    my_struct.num3 = my_struct.num3.wrapping_mul(2);
    mem.write(addr, my_struct);

    let short_text = mem.read_string(my_struct.short_text, 256);
    println!("short_text: {}, text len: {}", short_text, short_text.len());
    mem.write_buffer(b":p\0", my_struct.long_text);
}