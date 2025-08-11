/**
 * This is the “target” program, which is 
 * used to test memory reading and writing. 
 * The program declares a structure and, 
 * upon pressing [Enter], continuously prints 
 * the structure’s fields in an infinite loop.
 * 
 * For convenience, in the examples the program
 * will be compiled under the name “ABC123”.
 */

#include<stdio.h>
#include<stddef.h>
#include<stdint.h>

typedef struct {
    int             num;
    char*           long_text;
    char*           short_text;
    unsigned short  num2;
    char            padding[16];
    int8_t          num3;
} MyStruct;

int main(void) {
    MyStruct my_struct = {
        .num = 10,
        .long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
        .short_text = "hello world",
        .num2 = 4321,
        .num3 = -123
    };

    MyStruct* my_struct_ptr = &my_struct;
    ptrdiff_t num_offset = (char*)&my_struct_ptr->num - (char*)my_struct_ptr;
    ptrdiff_t num2_offset = (char*)&my_struct_ptr->num2 - (char*)my_struct_ptr;
    ptrdiff_t num3_offset = (char*)&my_struct_ptr->num3 - (char*)my_struct_ptr;
    ptrdiff_t short_text_offset = (char*)&my_struct_ptr->short_text - (char*)my_struct_ptr;
    ptrdiff_t long_text_offset = (char*)&my_struct_ptr->long_text - (char*)my_struct_ptr;
    for(;;) {
        printf("\33[H\33[2J");
        printf("| %-12s | %-12s | %-10s |\n", "NAME", "VALUE", "OFFSET");
        printf("| %-12s | %12d | 0x%08lx |\n", "num",  my_struct.num,        num_offset);
        printf("| %-12s | %12d | 0x%08lx |\n", "num2", my_struct.num2,       num2_offset);
        printf("| %-12s | %12d | 0x%08lx |\n", "num3", my_struct.num3,       num3_offset);
        puts  ("|------------------------------------------|");
        printf("| %-12s | %.30s\n", "short_text:",  my_struct.short_text);
        printf("| %-12s | 0x%08lx\n", "offset:", short_text_offset);
        printf("| %-12s | %.64s...\n", "long_text:",   my_struct.long_text);
        printf("| %-12s | 0x%08lx\n", "offset:",  long_text_offset);
        puts  ("|------------------------------------------|");
        printf("struct ptr: %p\n", my_struct_ptr);

        scanf("%*c");
    }
}