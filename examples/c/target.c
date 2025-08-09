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
        .long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua",
        .short_text = "hello",
        .num2 = 4321,
        .num3 = -123
    };

    MyStruct* my_struct_ptr = &my_struct;
    void* num_offset = (void*)(&my_struct_ptr->num2);
    void* num2_offset = (void*)(&my_struct_ptr->num2);
    void* num3_offset = (void*)(&my_struct_ptr->num3);

    for(;;) {
        printf("\33[H\33[2Jnum:\t\t%d\nlong_text:\t%s\nshort_text:\t%s\nnum2:\t\t%d\nnum3:\t\t%d\n",
            my_struct.num,
            my_struct.long_text,
            my_struct.short_text,
            my_struct.num2,
            my_struct.num3
        );

        printf("struct ptr: %p\n", my_struct_ptr);
        printf("num offset: 0x%lx\n", num_offset - (void*)my_struct_ptr);
        printf("num2 offset: 0x%lx\n", num2_offset - (void*)my_struct_ptr);
        printf("num3 offset: 0x%lx\n", num3_offset - (void*)my_struct_ptr);
        scanf("%*c");
    }
}