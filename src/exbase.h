
#pragma once

#include<stdint.h>
#include<stddef.h>

typedef void* ProcessInfo;
typedef void* LibraryInfo;
typedef void* Process;

/**
 * ProcessInfo
 */

ProcessInfo get_process_info_list(const char* name, int* out_len);
// ProcessInfo process_info_from_pid(int pid);
unsigned int process_info_pid(ProcessInfo proc_info);
const char* process_info_name(ProcessInfo proc_info);
const char* process_info_cmd(ProcessInfo proc_info);
const char* process_info_exe(ProcessInfo proc_info);
#ifdef __linux__
Process process_info_attach_vfile(ProcessInfo proc_info);
#endif
Process process_info_attach(ProcessInfo proc_info);
void free_process_info_list(ProcessInfo processes, int len);
void free_process_info(ProcessInfo proc_info);

/**
 * LibraryInfo
 */

LibraryInfo process_info_get_libraries(ProcessInfo lib, int* out_len);
const char* library_info_bin(LibraryInfo lib);
const char* library_info_perms(LibraryInfo lib);
uintptr_t library_info_address(LibraryInfo lib);
size_t library_info_size(LibraryInfo lib);
void free_library_info_list(LibraryInfo libraries, int len);

/*
 * Открытый процесс (mem file, system calls)
 */

#ifdef __linux__
#define process_read_vfile(proc, obj, addr) \
    process_read_buffer_vfile(proc, (unsigned char*)&(obj), sizeof(obj), addr)
#define process_write_vfile(proc, obj, addr) \
    process_write_buffer_vfile(proc, (const unsigned char*)&(obj), sizeof(obj), addr)

void process_read_buffer_vfile(Process proc, unsigned char* buf, size_t size, uintptr_t addr);
void process_write_buffer_vfile(Process proc, const unsigned char* buf, size_t size, uintptr_t addr);
const char* process_read_string_vfile(Process proc, size_t max_len, uintptr_t addr);
void free_process_vfile(Process proc);
#endif

#define process_read(proc, obj, addr) \
    process_read_buffer(proc, (unsigned char*)&(obj), sizeof(obj), addr)
#define process_write(proc, obj, addr) \
    process_write_buffer(proc, (const unsigned char*)&(obj), sizeof(obj), addr)

void process_read_buffer(Process proc, unsigned char* buf, size_t size, uintptr_t addr);
void process_write_buffer(Process proc, const unsigned char* buf, size_t size, uintptr_t addr);
const char* process_read_string(Process proc, size_t max_len, uintptr_t addr);
void free_process(Process proc);

/**
 * free
 */
void free_cstring(const char* s);