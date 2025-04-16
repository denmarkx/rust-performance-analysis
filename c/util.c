#include <stdlib.h>

void print_array(int* array, size_t size) {
    for (size_t i = 0; i < size; i++) {
        printf("%d ", array[i]);
    }
    printf("\n");
}

/*
* Currently only used for sorting programs.
* If no arguments are supplied, it'll write the 1st and 2nd index of argv in-place.
* Stops and warns if arguments are illegal.
*/
void warn_arguments(int argc, char* argv[]) {
    // One argument is fine, we'll just pick something on our own.
    if (argc == 1) {
        argv[1] = "5"; // [0-5]
        argv[2] = "5"; // 5 iterations.
        return;
    }

    // Though the sorting programs will require a max random value and iteration count.
    if (argc < 3) {
        printf("Incorrect usage: ./program <random_value> <iteration_count>.\n");
        exit(1);
    }
}
