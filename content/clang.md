
### C

[Full example](../examples/c/main.c).

#### Launching the example

```
cd exbase/build
./runexample
```

#### Fetching the process and its modules

```c
#include"exbase.h"
```

```c
int out_len = 0;
ProcessInfo* proc_info_list = get_process_info_list("ABC123", &out_len);
ProcessInfo proc_info = *proc_info_list;

unsigned int pid = process_info_pid(proc_info);
const char* name = process_info_name(proc_info);
const char* exe = process_info_exe(proc_info);
printf("PID: %d\n", pid);
printf("Name: %s\n", name);
printf("Executable: %s\n\n", exe);

int out_len = 0;
ModuleInfo* modules = process_info_get_modules(proc_info, &out_len);

for (int i = 0; i < out_len; ++i) {
    ModuleInfo mod = *(modules + i);
    const char* name = module_info_name(mod);
    const char* perms = module_info_perms(mod);
    uintptr_t address = module_info_address(mod);
    size_t size = module_info_size(mod);
    printf("Name: %s\n", name);
    printf("Address: %p\n", (void*)address);
    printf("Size: %ld (bytes)\n\n", size);
}
```

#### Memory

```rs
Memory mem = open_syscall_mem(proc_info);
```

#### Pattern scan

Define the pattern to locate `my_struct_ptr`.

```c
// mov    rax,QWORD PTR [rip+0x2a51]        # 0x403040 <my_struct_ptr>
// mov    eax,DWORD PTR [rax]
// mov    rdx,QWORD PTR [rbp-0x8]
Pattern pat = pattern_new("48 8b 05 ? ? ? ? 8b ?");
```

Locate the memory segment containing `my_struct_ptr`, get its base address and size (see example above), then read its bytes and scan for the pattern.
Once we’ve found the pattern and calculated its offset from the base module, we can obtain a pointer to `my_struct_ptr`.

```c
// 00400000-00401000 r-xp 00000000 00:23 1730705                            /path/to/ABC123
// 00403000-00404000 rw-p 00002000 00:23 1730705                            /path/to/ABC123
unsigned char* buf = malloc(mod_size);
memory_read_buffer(mem, buf, mod_size, mod_address);

int out_results_len = 0;
uintptr_t* pattern_offsets = pattern_scan(pat, buf, SCAN_RANGE_SIZE, 0, &out_results_len);

uintptr_t pattern_offset = *pattern_offsets;
uintptr_t my_struct_ptr = relative_address(mem, SCAN_RANGE_START + pattern_offset, 3, 7);
```

#### Reading and writing

```c
int value;
memory_read(mem, value, addr + 0x18);
memory_write(mem, value, addr + 0x18);
```

Strings

```c
const char* short_text = memory_read_string(mem, 256, my_struct.short_text);
char new_text[] = "hello world";
memory_write_buffer(mem, (const unsigned char*)new_text, sizeof(new_text), my_struct.long_text);
```

Structures

```c
typedef struct {
    int             num;
    char*           long_text;
    char*           short_text;
    unsigned short  num2;
    char            padding[16];
    int8_t          num3;
} MyStruct;
```

```c
MyStruct my_struct;
memory_read(mem, my_struct, addr);
memory_write(mem, my_struct, addr);
```