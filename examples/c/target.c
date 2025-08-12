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
#include<stdlib.h>
#include<string.h>
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

MyStruct* my_struct_ptr = NULL;

void init_struct() {
    my_struct_ptr = malloc(sizeof(MyStruct));
    my_struct_ptr->num = 10;
    my_struct_ptr->num2 = 4321;
    my_struct_ptr->num3 = -123;

    const char* text1 = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    const char* text2 = "Hello";
    my_struct_ptr->long_text = malloc(strlen(text1) + 1);
    my_struct_ptr->short_text = malloc(strlen(text2) + 1);
    strcpy(my_struct_ptr->long_text, text1);
    strcpy(my_struct_ptr->short_text, text2);
}

int main(void) {
    init_struct();
    ptrdiff_t num_offset = (char*)&my_struct_ptr->num - (char*)my_struct_ptr;
    ptrdiff_t num2_offset = (char*)&my_struct_ptr->num2 - (char*)my_struct_ptr;
    ptrdiff_t num3_offset = (char*)&my_struct_ptr->num3 - (char*)my_struct_ptr;
    ptrdiff_t short_text_offset = (char*)&my_struct_ptr->short_text - (char*)my_struct_ptr;
    ptrdiff_t long_text_offset = (char*)&my_struct_ptr->long_text - (char*)my_struct_ptr;
    
    for(;;) {
        printf("\33[H\33[2J");
        printf("| %-12s | %-12s | %-10s |\n", "NAME", "VALUE", "OFFSET");
        printf("| %-12s | %12d | 0x%08lx |\n", "num", my_struct_ptr->num, num_offset);
        printf("| %-12s | %12d | 0x%08lx |\n", "num2", my_struct_ptr->num2, num2_offset);
        printf("| %-12s | %12d | 0x%08lx |\n", "num3", my_struct_ptr->num3, num3_offset);
        puts  ("|------------------------------------------|");
        printf("| %-12s | %.30s\n", "short_text:",  my_struct_ptr->short_text);
        printf("| %-12s | 0x%08lx\n", "offset:", short_text_offset);
        printf("| %-12s | %.64s...\n", "long_text:",   my_struct_ptr->long_text);
        printf("| %-12s | 0x%08lx\n", "offset:",  long_text_offset);
        puts  ("|------------------------------------------|");
        printf("struct ptr: %p\n", my_struct_ptr);

        scanf("%*c");
    }
}