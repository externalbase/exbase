#include<stdio.h>
#include<stddef.h>
#include<string.h>
#include<stdlib.h>
#include"exbase.h"

typedef struct {
    int             num;
    uintptr_t       long_text;
    uintptr_t       short_text;
    unsigned short  num2;
    char            padding[16];
    int8_t          num3;
} MyStruct;

uintptr_t relative_address(Memory mem, uintptr_t pattern_addr, size_t offset, size_t inst_lenght);

void print_process_info(ProcessInfo proc);
void print_modules(ModuleInfo mod);
void read_write_field(Memory mem, uintptr_t my_struct_ptr);
void read_write_struct(Memory mem, uintptr_t my_struct_ptr);

uintptr_t SCAN_RANGE_START = 0;
uintptr_t SCAN_RANGE_SIZE = 0;

const char* PROCESS_NAME =
#if defined(_WIN32) || defined(_WIN64)
"ABC123.exe";
#else
"ABC123";
#endif

int main(int argc, char** argv) {
    int out_len = 0;
    ProcessInfo* proc_info_list = get_process_info_list(PROCESS_NAME, &out_len);

    if (!proc_info_list) {
        puts("Не найдено ни одного процесса");
        return 1;
    }
    if (out_len > 1) {
        printf("Найдено %d процесса(ов)\n", out_len);
        for (int i = 0; i < out_len; ++i) {
            printf("%d. - PID: %d\n", i, process_info_pid(proc_info_list[i]));
        }
        free_process_info_list(proc_info_list, out_len);
        return 1;
    }

    ProcessInfo proc_info = *proc_info_list;

    print_process_info(proc_info);
    print_modules(proc_info);

    Memory mem = open_syscall_mem(proc_info);
    if (!mem) {
        printf("failed\n");
        free_process_info_list(proc_info_list, out_len);
        return 1;
    }

    Pattern pat = pattern_new("48 8b 05 ? ? ? ? 8b ?");
    if (!pat) {
        printf("Failed to parse\n");
        return 1;
    }

    unsigned char* buf = malloc(SCAN_RANGE_SIZE);
    memory_read_buffer(mem, buf, SCAN_RANGE_SIZE, SCAN_RANGE_START);

    int out_results_len = 0;
    uintptr_t* pattern_offsets = pattern_scan(pat, buf, SCAN_RANGE_SIZE, 0, &out_results_len);
    if (!pattern_offsets) {
        printf("Failed\n");
        return 1;
    }
    uintptr_t pat_offset = *pattern_offsets;

    uintptr_t my_struct_ptr = relative_address(mem, SCAN_RANGE_START + pat_offset, 3, 7);

    free_pattern(pat);
    free_pattern_offsets(pattern_offsets, out_results_len);

    free_process_info_list(proc_info_list, out_len);

    read_write_field(mem, my_struct_ptr);
    read_write_struct(mem, my_struct_ptr);

    return 0;
}

void print_process_info(ProcessInfo proc_info) {
    unsigned int pid = process_info_pid(proc_info);
    const char* name = process_info_name(proc_info);
    const char* exe = process_info_exe(proc_info);

    printf("PID: %d\n", pid);
    printf("Name: %s\n", name);
    printf("Executable: %s\n\n", exe);

    free_cstring(name);
    free_cstring(exe);
}

void print_modules(ProcessInfo proc_info) {
    int out_len = 0;
    ModuleInfo* modules = process_info_get_modules(proc_info, &out_len);
    if (!modules) {
        puts("не удалось получить библиотеки\n");
        return;
    }
    
    for (int i = 0; i < out_len; ++i) {
        ModuleInfo mod = *(modules + i);
        const char* name = module_info_name(mod);
        const char* perms = module_info_perms(mod);
        uintptr_t address = module_info_address(mod);
        size_t size = module_info_size(mod);

        printf("Name: %s\n", name);
        printf("Address: %p\n", (void*)address);
        printf("Size: %ld (bytes)\n\n", size);

        if (strcmp(PROCESS_NAME, name) == 0) {
            SCAN_RANGE_START = address;
            SCAN_RANGE_SIZE = size;
        }

        free_cstring(name);
        free_cstring(perms);
    }

    free_module_info_list(modules, out_len);
}

void read_write_field(Memory mem, uintptr_t my_struct_ptr) {
    uintptr_t addr = 0;
    memory_read(mem, addr, my_struct_ptr);
    int num2;
    memory_read(mem, num2, addr + 0x18);
    num2 *= -1;
    memory_write(mem, num2, addr + 0x18);
}

void read_write_struct(Memory mem, uintptr_t my_struct_ptr) {
    uintptr_t addr = 0;
    memory_read(mem, addr, my_struct_ptr);
    MyStruct my_struct;
    memory_read(mem, my_struct, addr);
    my_struct.num += 3;
    my_struct.num3 *= 2;
    memory_write(mem, my_struct, addr);

    const char* short_text = memory_read_string(mem, 256, my_struct.short_text);
    printf("short_text: %s, text len: %ld\n", short_text, strlen(short_text));
    free_cstring(short_text);

    char new_text[] = ":p";
    memory_write_buffer(mem, (const unsigned char*)new_text, sizeof(new_text), my_struct.long_text);
}

uintptr_t relative_address(Memory mem, uintptr_t pattern_addr, size_t offset, size_t inst_lenght) {
    int rip_rel = 0;
    memory_read(mem, rip_rel, pattern_addr + offset);
    return pattern_addr + inst_lenght + rip_rel;
}