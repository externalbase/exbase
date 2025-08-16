
#pragma once

#include<stdint.h>
#include<stddef.h>

typedef void* ProcessInfo;
typedef void* LibraryInfo;
typedef void* Memory;
typedef void* MemoryVFile;

/**
 * ProcessInfo
 */

ProcessInfo get_process_info_list(const char* name, int* out_len);
unsigned int process_info_pid(ProcessInfo proc_info);
const char* process_info_name(ProcessInfo proc_info);
const char* process_info_exe(ProcessInfo proc_info);
void free_process_info_list(ProcessInfo processes, int len);
void free_process_info(ProcessInfo proc_info);

/**
 * LibraryInfo
 */

LibraryInfo process_info_get_libraries(ProcessInfo lib, int* out_len);
const char* library_info_name(LibraryInfo lib);
const char* library_info_perms(LibraryInfo lib);
uintptr_t library_info_address(LibraryInfo lib);
size_t library_info_size(LibraryInfo lib);
void free_library_info_list(LibraryInfo libraries, int len);

/*
 * Memory
 */

#ifdef __linux__
MemoryVFile open_vfile_mem(ProcessInfo proc_info);
#define memory_read_vfile(mem, obj, addr) \
    memory_read_buffer_vfile(mem, (unsigned char*)&(obj), sizeof(obj), addr)
#define memory_write_vfile(mem, obj, addr) \
    memory_write_buffer_vfile(mem, (const unsigned char*)&(obj), sizeof(obj), addr)

void memory_read_buffer_vfile(MemoryVFile mem, unsigned char* buf, size_t size, uintptr_t addr);
void memory_write_buffer_vfile(MemoryVFile proc, const unsigned char* buf, size_t size, uintptr_t addr);
const char* memory_read_string_vfile(MemoryVFile proc, size_t max_len, uintptr_t addr);
void free_memory_obj_vfile(MemoryVFile mem);
#endif

Memory open_syscall_mem(ProcessInfo proc_info);
#define memory_read(mem, obj, addr) \
    memory_read_buffer(mem, (unsigned char*)&(obj), sizeof(obj), addr)
#define memory_write(mem, obj, addr) \
    memory_write_buffer(mem, (const unsigned char*)&(obj), sizeof(obj), addr)

void memory_read_buffer(Memory mem, unsigned char* buf, size_t size, uintptr_t addr);
void memory_write_buffer(Memory proc, const unsigned char* buf, size_t size, uintptr_t addr);
const char* memory_read_string(Memory proc, size_t max_len, uintptr_t addr);
void free_memory_obj(Memory mem);

/**
 * free
 */
void free_cstring(const char* s);

// w/r/buf, string
// free_memory_obj