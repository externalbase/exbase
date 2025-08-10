#include<stdio.h>
#include"exbase.h" // ${workspaceFolder}/../../src/exbase.h

void display_process_info(ProcessInfo proc);
void display_libraries(LibraryInfo lib);

int main(int argc, char** argv) {
    int out_len = 0;
    ProcessInfo* processes = get_process_info_list("ABC123", &out_len);
    if (!processes) {
        puts("Не найдено ни одного процесса\n");
        return 1;
    }

    if (out_len > 1) {
        printf("Найдено %d процесса(ов)\n", out_len);
        for (int i = 0; i < out_len; ++i) {
            printf("%d - PID: %d\n", i, process_info_pid(processes[i]));
        }
        free_process_info_list(processes, out_len);
        return 1;
    }

    ProcessInfo proc = *processes;

    display_process_info(proc);
    display_libraries(proc);

    free_process_info_list(processes, out_len);
    return 0;
}

void display_process_info(ProcessInfo proc) {
    unsigned int pid = process_info_pid(proc);
    const char* name = process_info_name(proc);
    const char* cmd = process_info_cmd(proc);
    const char* exe = process_info_exe(proc);

    printf("PID: %d\n", pid);
    printf("Name: %s\n", name);
    printf("Cmd: %s\n", cmd);
    printf("Executable: %s\n\n", exe);

    // Отчищаем память
    free_cstring(name);
    free_cstring(cmd);
    free_cstring(exe);
}

void display_libraries(ProcessInfo proc) {
    int out_len = 0;
    LibraryInfo* libraries = process_info_get_libraries(proc, &out_len);
    if (!libraries) {
        puts("не удалось получить библиотеки\n");
        return;
    }
    
    for (int i = 0; i < out_len; ++i) {
        LibraryInfo lib = *(libraries + i);
        const char* bin = library_info_bin(lib);
        const char* perms = library_info_perms(lib);
        uintptr_t address = library_info_address(lib);
        uintptr_t size = library_info_size(lib);

        printf("Binary path: %s\n", bin);
        printf("Permissions: %s\n", perms);
        printf("Address: %p\n", address);
        printf("Size: %d (bytes)\n\n", size);
    }

    // free bin, perms

    free_library_info_list(libraries, out_len);
}