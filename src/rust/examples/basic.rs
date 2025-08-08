use exbase::{MemoryAccessor, Process, ProcessInfo};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MyStruct {
    pub num: i32,
    pub long_text: usize,
    pub short_text: usize,
    pub num2: u16,
    pub _padding: [u8; 16],
    pub num3: i8,
}

pub fn main() {
    let processes = ProcessInfo::processes_by_name("ABC123").expect("Failed to get processes");

    if processes.len() > 1 {
        println!("Warning: More than one process found");
    }

    let process_info = processes.iter().next().expect("Not found");

    for lib in process_info.get_libraries().unwrap() {
        if lib.can_read() {
            println!("Address: 0x{:x}\tLocation: {}", lib.get_address(), lib.get_bin())
        }
    }

    let struct_ptr = read_ptr("my struct pointer: ");
    println!("ptr: 0x{:x}", struct_ptr);

    let pid = process_info.get_pid();
    let process = process_info.attach(exbase::StreamMem::new(pid).unwrap());

    let my_struct: MyStruct = process.memory.read(struct_ptr);
    print_struct(&process.memory, &my_struct);

    let new_struct = MyStruct {
        num: -20,
        long_text: 0, // null
        short_text: 0, // null
        num2: 1234,
        _padding: [0u8; 16],
        num3: 33,
    };
    process.memory.write(struct_ptr, new_struct);
    process.memory.write(struct_ptr + 0x18, 3333);
    process.memory.write(struct_ptr + 0x2a, -44);

    let my_struct: MyStruct = process.memory.read(struct_ptr);
    print_struct(&process.memory, &my_struct);
}

pub fn print_struct(mem: &impl MemoryAccessor, my_struct: &MyStruct) {
    println!("num: {}", my_struct.num);
    println!("long_text: {}", mem.read_string(my_struct.long_text, 64));
    println!("short_text: {}", mem.read_string(my_struct.short_text, 64));
    println!("num2: {}", my_struct.num2);
    println!("_padding: {} bytes", my_struct._padding.len());
    println!("num3: {}\n", my_struct.num3);
}

pub fn read_ptr(label: &str) -> usize {
    use std::io::Write;

    print!("{}", label);
    std::io::stdout().flush().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    usize::from_str_radix(s.trim_start_matches("0x").trim_end(), 16).unwrap()
}

// pub fn e2() {
//     let _proc = ProcessInfo::from_pid(1234).expect("Not found or permission denied");
// }