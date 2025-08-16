#include<stdio.h>
#include<stddef.h>
#include<string.h>
#include"exbase.h"

typedef struct {
    int             num;
    uintptr_t       long_text;
    uintptr_t       short_text;
    unsigned short  num2;
    char            padding[16];
    int8_t          num3;
} MyStruct;

void print_process_info(ProcessInfo proc);
void print_libraries(LibraryInfo lib);
void read_write_field(Memory mem);
void read_write_struct(Memory mem);

uintptr_t HEAP_ADDR = 0;

int main(int argc, char** argv) {
    int out_len = 0;
    ProcessInfo* proc_info_list = get_process_info_list("ABC123", &out_len);

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
    print_libraries(proc_info);

    Memory mem = open_syscall_mem(proc_info);
    if (!mem) {
        printf("failed\n");
        free_process_info_list(proc_info_list, out_len);
        return 1;
    }

    free_process_info_list(proc_info_list, out_len);

    read_write_field(mem);
    read_write_struct(mem);

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

void print_libraries(ProcessInfo proc_info) {
    int out_len = 0;
    LibraryInfo* libraries = process_info_get_libraries(proc_info, &out_len);
    if (!libraries) {
        puts("не удалось получить библиотеки\n");
        return;
    }
    
    for (int i = 0; i < out_len; ++i) {
        LibraryInfo lib = *(libraries + i);
        const char* name = library_info_name(lib);
        const char* perms = library_info_perms(lib);
        uintptr_t address = library_info_address(lib);
        size_t size = library_info_size(lib);

        printf("Name: %s\n", name);
        printf("Address: %p\n", (void*)address);
        printf("Size: %ld (bytes)\n\n", size);

        if (strcmp("[heap]", name) == 0) {
            HEAP_ADDR = address;
        }

        free_cstring(name);
        free_cstring(perms);
    }

    free_library_info_list(libraries, out_len);
}

void read_write_field(Memory mem) {
    uintptr_t addr = HEAP_ADDR + 0x2a0;
    int num2;
    memory_read(mem, num2, addr + 0x18);
    num2 *= -1;
    memory_write(mem, num2, addr + 0x18);
}

void read_write_struct(Memory mem) {
    uintptr_t addr = HEAP_ADDR + 0x2a0;
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