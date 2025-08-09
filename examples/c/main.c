#include<stdio.h>
#include"exbase.h"

int main(int argc, char** argv) {
    int out_len = 0;
    ProcessInfo* processes = get_process_info_list("bash", &out_len);
    if (!processes) {
        return 1;
    }

    printf("ptr: %p\n", processes);
    printf("Процессы получены (%d)\n", out_len);
    for (int i = 0; i < out_len; ++i) {
        printf("%p - Перечисляем (%d) pid: %d\n", *&processes[i], i, process_info_pid(*&processes[i]));
    }

    free_process_info_list(processes, out_len);
    return 0;
}