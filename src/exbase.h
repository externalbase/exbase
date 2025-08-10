
#pragma once

#include<stdint.h>

typedef void* ProcessInfo;
typedef void* LibraryInfo;

// ProcessInfo
ProcessInfo get_process_info_list(const char* name, int* out_len);
unsigned int process_info_pid(ProcessInfo proc);
unsigned const char* process_info_name(ProcessInfo proc);
unsigned const char* process_info_cmd(ProcessInfo proc);
unsigned const char* process_info_exe(ProcessInfo proc);
void free_process_info_list(ProcessInfo processes, int len);

// LibraryInfo
LibraryInfo process_info_get_libraries(LibraryInfo lib, int* out_len);
unsigned const char* library_info_bin(LibraryInfo lib);
unsigned const char* library_info_perms(LibraryInfo lib);
uintptr_t library_info_address(LibraryInfo lib);
uintptr_t library_info_size(LibraryInfo lib);
void free_library_info_list(LibraryInfo libraries, int len);

void free_cstring(const char* s);