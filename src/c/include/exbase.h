
#pragma once

typedef const void* ProcessInfo;

ProcessInfo get_process_info_list(const char* name, int* out_len);
unsigned int process_info_pid(ProcessInfo proc);
void free_process_info_list(ProcessInfo processes);